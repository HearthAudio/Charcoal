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
use log::error;
use nanoid::nanoid;
use tokio::sync::broadcast::Sender;
use tokio::sync::{broadcast, Mutex};
use crate::background::init_background;
use crate::background::processor::IPCData;
use crate::connector::{initialize_client, initialize_producer};
use crate::logger::setup_logger;

mod connector;
pub mod actions;
mod logger;
pub mod serenity;
mod constants;
mod background;

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
    tx: Sender<IPCData>,
    worker_id: Option<String>,
    job_id:  Option<String>,
    guild_id:  Option<String>,
    channel_id:  Option<String>
}

impl PlayerObject {
    pub async fn new(charcoal: Arc<Mutex<Charcoal>>) -> Self {
        PlayerObject {
            tx: charcoal.lock().await.tx.clone(),
            worker_id: None,
            job_id: None,
            guild_id: None,
            channel_id: None,
        }
    }
}

pub struct Charcoal {
    tx: Sender<IPCData>
}

pub async fn init_charcoal(broker: String) -> Arc<Mutex<Charcoal>>  {
    let brokers = vec![broker];
    let (tx, mut rx) = broadcast::channel(16);
    init_background(tx.clone(),rx,brokers);
    return Arc::new(Mutex::new(Charcoal {
        tx
    }));
}
