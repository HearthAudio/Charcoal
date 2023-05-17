use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc};
use std::thread::sleep;
use std::time::Duration;
use ::serenity::Client;
use async_trait::async_trait;
use futures::SinkExt;
use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use lazy_static::lazy_static;
use log::error;
use nanoid::nanoid;
use tokio::sync::{broadcast, Mutex};
use crate::background::init_background;
use crate::background::processor::IPCData;

mod connector;
pub mod actions;
pub mod serenity;
mod background;

lazy_static! {
    pub static ref PRODUCER: Mutex<Option<Producer>> = Mutex::new(None);
    pub static ref CONSUMER: Mutex<Option<Consumer>> = Mutex::new(None);
}

#[derive(Clone,Debug)]
pub struct JobResult {
    pub job_id: String,
    pub worker_id: String
}

pub struct PlayerObject {
    worker_id: Option<String>,
    job_id:  Option<String>,
    guild_id:  Option<String>,
    channel_id:  Option<String>
}

impl PlayerObject {
    pub async fn new() -> Self {
        PlayerObject {
            worker_id: None,
            job_id: None,
            guild_id: Some(guild_id.clone()),
            channel_id: None,
        }
    }
}

pub struct Charcoal {
    pub players: HashMap<String,PlayerObject> // Guild ID to PlayerObject
}

impl Charcoal {
    pub fn get_player(&mut self,guild_id: &String) -> &mut PlayerObject {
        return self.players.get_mut(guild_id).unwrap();
    }
}

pub async fn init_charcoal(broker: String) -> Arc<Mutex<Charcoal>>  {
    let brokers = vec![broker];

    .with_topic(String::from("communication"))
    .create()
    .unwrap();

    *PRODUCER.lock().await = Some(producer);
    *CONSUMER.lock().await = Some(consumer);

    return Arc::new(Mutex::new(Charcoal {
        players: HashMap::new()
    }));
}
