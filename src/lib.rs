use std::collections::HashMap;

use std::sync::{Arc};





use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use lazy_static::lazy_static;


use tokio::sync::Mutex;
use crate::connector::{initialize_client, initialize_producer};


mod connector;
pub mod actions;
pub mod serenity;
mod constants;

lazy_static! {
    pub static ref PRODUCER: Mutex<Option<Producer>> = Mutex::new(None);
    pub static ref CONSUMER: Mutex<Option<Consumer>> = Mutex::new(None);
}

#[derive(Clone,Debug)]
pub enum StandardActionType {
    JoinChannel
}

#[derive(Clone,Debug)]
pub enum InfrastructureType {
    JoinChannelResult
}

#[derive(Clone,Debug)]
pub enum InternalIPCType {
    DWCAction(DWCActionType),
    StandardAction(StandardActionType),
    Infrastructure(InfrastructureType)
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
            guild_id: None,
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
    //TODO: Sort this mess out
    let producer : Producer = initialize_producer(initialize_client(&brokers));
    let consumer = Consumer::from_client(initialize_client(&brokers))

    .with_topic(String::from("communication"))
    .create()
    .unwrap();

    *PRODUCER.lock().await = Some(producer);
    *CONSUMER.lock().await = Some(consumer);

    return Arc::new(Mutex::new(Charcoal {
        players: HashMap::new()
    }));
}
