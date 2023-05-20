//! Charcoal is a client-library for Hearth that makes it easy to use Hearth with Rust.
//! See Examples in the Github repo in the sub-folder examples/
use std::collections::HashMap;
use std::sync::{Arc};
use futures::SinkExt;
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use lazy_static::lazy_static;
use tokio::sync::{broadcast, Mutex};
use tokio::sync::broadcast::{Receiver, Sender};
use crate::background::processor::init_processor;
use crate::connector::{initialize_client, initialize_producer};
mod connector;
pub mod actions;
pub mod serenity;
pub(crate) mod background;

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
    guild_id:  Option<String>,
}

impl PlayerObject {
    /// Creates a new Player Object that can then be joined to channel and used to playback audio
    pub async fn new() -> Self {
        PlayerObject {
            worker_id: None,
            job_id: None,
            guild_id: None,
        }
    }
}

/// Stores Charcoal instance
pub struct Charcoal {
    pub players: HashMap<String,PlayerObject> // Guild ID to PlayerObject
}

impl Charcoal {
    pub fn get_player(&mut self,guild_id: &String) -> Option<&mut PlayerObject> {
        return self.players.get_mut(guild_id);
    }
}

pub struct SSLConfig {
    pub ssl_key: String,
    pub ssl_ca: String,
    pub ssl_cert: String,
}

pub struct CharcoalConfig {
    pub ssl: Option<SSLConfig>,
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

    *PRODUCER.lock().await = Some(producer);
    *CONSUMER.lock().await = Some(consumer);
    let mut g_tx: Arc<Option<Sender<String>>> = Arc::new(None);
    let mut t_g_tx = g_tx.clone();
    tokio::task::spawn(async move {
        let (tx, mut rx) = broadcast::channel(16);
        t_g_tx = Arc::new(Some(tx.clone()).clone());
        init_processor(rx).await;
    });

    let mut x = g_tx.clone();
    x.as_ref().unwrap();

    return Arc::new(Mutex::new(Charcoal {
        players: HashMap::new()
    }));
}
