//! Charcoal is a client-library for Hearth that makes it easy to use Hearth with Rust.
//! See Examples in the Github repo in the sub-folder examples/
use std::collections::HashMap;
use std::sync::{Arc};
use lazy_static::lazy_static;
use nanoid::nanoid;
use tokio::sync::Mutex;
use crate::connector::{initialize_consume, initialize_producer};
mod connector;
pub mod actions;
pub mod serenity;
mod constants;

use rdkafka::consumer::{StreamConsumer};
use rdkafka::producer::{FutureProducer};

lazy_static! {
    pub(crate) static ref PRODUCER: Mutex<Option<FutureProducer>> = Mutex::new(None);
    pub(crate) static ref CONSUMER: Mutex<Option<StreamConsumer>> = Mutex::new(None);
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
    pub fn get_player(&mut self,guild_id: &String) -> &mut PlayerObject {
        return self.players.get_mut(guild_id).unwrap();
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

    let consumer = initialize_consume(&broker,&config,&nanoid!()).await;

    let producer = initialize_producer(&broker, &config);

    *PRODUCER.lock().await = Some(producer);
    *CONSUMER.lock().await = Some(consumer);

    return Arc::new(Mutex::new(Charcoal {
        players: HashMap::new()
    }));
}
