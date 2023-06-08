//! Charcoal is a client-library for Hearth that makes it easy to use Hearth with Rust.
//! See Examples in the Github repo [here](https://github.com/Hearth-Industries/Charcoal/tree/main/examples)

use crate::actions::channel_manager::CreateJobError;
use crate::background::processor::{init_processor, IPCData};
use hearth_interconnect::messages::Message;
use lazy_static::lazy_static;
use log::{error, info};
use rdkafka::producer::FutureProducer;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;
use futures::StreamExt;
use prokio::time;
use std::sync::Mutex;
use kanal::{Receiver, Sender};
use prokio::time::sleep;

pub mod actions;
pub mod background;
pub(crate) mod constants;
mod helpers;
pub mod serenity;

use crate::background::connector::{initialize_client, initialize_producer};
use rdkafka::consumer::StreamConsumer;

/// Represents an instance in a voice channel
pub struct PlayerObject {
    worker_id: Arc<RwLock<Option<String>>>,
    job_id: Arc<RwLock<Option<String>>>,
    guild_id: String,
    tx: Arc<Sender<IPCData>>,
    rx: Arc<Receiver<IPCData>>,
    bg_com_tx: Sender<IPCData>,
}

impl PlayerObject {
    /// Creates a new Player Object that can then be joined to channel and used to playback audio
    pub async fn new(guild_id: String, com_tx: Sender<IPCData>) -> Result<Self, CreateJobError> {
        let (tx, rx) = kanal::bounded(60);

        let handler = PlayerObject {
            worker_id: Arc::new(RwLock::new(None)),
            job_id: Arc::new(RwLock::new(None)),
            guild_id,
            tx: Arc::new(tx),
            rx: Arc::new(rx),
            bg_com_tx: com_tx,
        };

        Ok(handler)
    }
}

/// Stores Charcoal instance
pub struct Charcoal {
    pub players: Arc<RwLock<HashMap<String, PlayerObject>>>, // Guild ID to PlayerObject
    pub to_bg_tx: Sender<IPCData>,
    from_bg_rx: Receiver<IPCData>
}

impl Charcoal {
    fn start_global_checker(&mut self) {
        info!("Started global data checker!");
        let t_players = self.players.clone();
        let from_bg_rx_t = self.from_bg_rx.clone();
        prokio::spawn_local(async move {
            loop {
                sleep(Duration::from_millis(250)).await;
                let catch = from_bg_rx_t.try_recv();
                match catch {
                    Ok(c) => {
                        if c.is_some() {
                            if let IPCData::FromBackground(bg) = c.unwrap() {
                                match bg.message {
                                    Message::ExternalJobExpired(je) => {
                                        info!("Job Expired: {}", je.job_id);
                                        let mut t_p_write = t_players.write().unwrap();
                                        t_p_write.remove(&je.guild_id);
                                    }
                                    Message::WorkerShutdownAlert(shutdown_alert) => {
                                        info!("Worker shutdown! Cancelling Players!");
                                        let mut t_p_write = t_players.write().unwrap();
                                        for job_id in shutdown_alert.affected_guild_ids {
                                            t_p_write.remove(&job_id);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Err(e) => {} //TODO: Handle
                }
            }

        });
    }
}

#[derive(Clone)]
/// Stores SSL Config for Kafka
pub struct SSLConfig {
    /// Path to the SSL key file
    pub ssl_key: String,
    /// Path to the SSL CA file
    pub ssl_ca: String,
    /// Path to the SSL cert file
    pub ssl_cert: String,
}

#[derive(Clone)]
pub struct SASLConfig {
    /// Kafka Username
    pub kafka_username: String,
    /// Kafka Password
    pub kafka_password: String,
}

#[derive(Clone)]
/// Configuration for charcoal
pub struct CharcoalConfig {
    /// Configure SSl for kafka. If left as None no SSL is configured
    pub ssl: Option<SSLConfig>,
    /// Configure SASL/Password and Username Based Authentication for Kafka. If left as None no SASL is configured
    pub sasl: Option<SASLConfig>,
    /// Kafka topic to connect to. This should be the same one the hearth server(s) are on.
    pub kafka_topic: String,
}

/// Initializes Charcoal Instance
pub async fn init_charcoal(broker: String, config: CharcoalConfig) -> Arc<Mutex<Charcoal>> {
    // This isn't great we should really switch to rdkafka instead of kafka

    let consumer = initialize_client(&broker, &config).await;

    let producer = initialize_producer(&broker, &config);

    let (to_bg_tx, to_bg_rx) = kanal::unbounded();
    let (from_bg_tx,from_bg_rx) = kanal::unbounded();

    prokio::spawn_local(async move {
        init_processor(to_bg_rx, from_bg_tx, consumer, producer, config).await;
    });

    let mut c_instance = Charcoal {
        players: Arc::new(RwLock::new(HashMap::new())),
        to_bg_tx,
        from_bg_rx
    };

    c_instance.start_global_checker(); // Start checking for expired jobs

    Arc::new(Mutex::new(c_instance))
}
