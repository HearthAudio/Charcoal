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
use tokio::sync::Mutex;
use crate::connector::{initialize_client, initialize_producer};
use crate::logger::setup_logger;

mod connector;
pub mod actions;
mod logger;
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

#[derive(Clone,Debug)]
pub struct InternalIPC {
    action: InternalIPCType,
    dwc: Option<DirectWorkerCommunication>,
    worker_id: Option<String>,
    job_id: Option<String>,
    queue_job_request: Option<JobRequest>,
    job_result: Option<JobResult>,
    request_id: Option<String>
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
    let mut consumer = Consumer::from_client(initialize_client(&brokers))

    .with_topic(String::from("communication"))
    .create()
    .unwrap();

    *PRODUCER.lock().await = Some(producer);
    *CONSUMER.lock().await = Some(consumer);

    return Arc::new(Mutex::new(Charcoal {
        players: HashMap::new()
    }));
}
