//! Charcoal is a client-library for Hearth that makes it easy to use Hearth with Rust.
//! See Examples in the Github repo [here](https://github.com/Hearth-Industries/Charcoal/tree/main/examples)
use std::collections::HashMap;
use std::sync::{Arc};
use std::time::Duration;
use hearth_interconnect::messages::Message;
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use lazy_static::lazy_static;
use log::error;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time;
use crate::background::processor::{init_processor, IPCData};
use crate::connector::{initialize_client, initialize_producer};
mod connector;
pub mod actions;
pub mod serenity;
pub mod background;

lazy_static! {
    pub(crate) static ref PRODUCER: Mutex<Option<Producer>> = Mutex::new(None);
    pub(crate) static ref CONSUMER: Mutex<Option<Consumer>> = Mutex::new(None);
    pub(crate) static ref TX: Mutex<Option<Sender<String>>> = Mutex::new(None);
    pub(crate) static ref RX: Mutex<Option<Receiver<String>>> = Mutex::new(None);
}

/// Represents an instance in a voice channel
pub struct PlayerObject {
    worker_id: Option<String>,
    job_id:  Option<String>,
    guild_id:  String,
    tx: Arc<Sender<IPCData>>,
    rx: Receiver<IPCData>,
    bg_com_tx: Sender<IPCData>
}

impl PlayerObject {
    /// Creates a new Player Object that can then be joined to channel and used to playback audio
    pub async fn new(guild_id: String,com_tx: Sender<IPCData>) -> Self {
        let (tx, mut rx) = broadcast::channel(16);

        PlayerObject {
            worker_id: None,
            job_id: None,
            guild_id,
            tx: Arc::new(tx),
            rx: rx,
            bg_com_tx: com_tx
        }
    }
}

/// Stores Charcoal instance
pub struct Charcoal {
    pub players: Arc<RwLock<HashMap<String,PlayerObject>>>, // Guild ID to PlayerObject
    pub tx: Sender<IPCData>,
    pub rx: Receiver<IPCData>
}

impl Charcoal {
    fn start_expiration_checker(&mut self) {
        let mut rxx = self.tx.subscribe();
        let mut t_players = self.players.clone();
        tokio::task::spawn(async move {
            // let mut interval = time::interval(Duration::from_secs(1));

            loop {
                // interval.tick().await;
                let catch = rxx.recv().await;
                match catch {
                    Ok(c) => {
                        match c {
                            IPCData::FromBackground(bg) => {
                                match bg.message {
                                    Message::ExternalJobExpired(je) => {
                                        let mut t_p_write = t_players.write().await;
                                        t_p_write.remove(&je.guild_id);
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    Err(e) => {
                        error!("Failed to receive expiration check with error: {}",e);
                    }
                }
            }
        });
    }
}

/// Stores SSL Config for Kafka
pub struct SSLConfig {
    /// Path to the SSL key file
    pub ssl_key: String,
    /// Path to the SSL CA file
    pub ssl_ca: String,
    /// Path to the SSL cert file
    pub ssl_cert: String,
}

/// Configuration for charcoal
pub struct CharcoalConfig {
    /// Configure SSl for kafka. If left as None no SSL is configured
    pub ssl: Option<SSLConfig>,
    /// Kafka topic to connect to. This should be the same one the hearth server(s) are on.
    pub kafka_topic: String
}

/// Initializes Charcoal Instance
pub async fn init_charcoal(broker: String,config: CharcoalConfig) -> Arc<Mutex<Charcoal>>  {
    let brokers = vec![broker];

    // This isn't great we should really switch to rdkafka instead of kafka

    let consumer = Consumer::from_client(initialize_client(&brokers,&config))
        .with_topic(config.kafka_topic.clone())
        .create()
        .unwrap();

    let producer : Producer = initialize_producer(initialize_client(&brokers,&config));

    let (tx, mut rx) = broadcast::channel(16);

    let sub_tx = tx.clone();
    let global_rx = tx.subscribe();

    tokio::task::spawn(async move {
        init_processor(rx,sub_tx,consumer,producer).await;
    });

    Arc::new(Mutex::new(Charcoal {
        players: Arc::new(RwLock::new(HashMap::new())),
        tx,
        rx: global_rx
    }))
}
