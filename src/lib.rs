use std::ops::Deref;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use async_trait::async_trait;
use futures::SinkExt;
use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::producer::Producer;
use log::error;
use nanoid::nanoid;
use crate::connector::{initialize_client, initialize_producer};
use crate::logger::setup_logger;

mod connector;
pub mod actions;
mod logger;
pub mod serenity;

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
    producer: &'static Producer,
    worker_id: Option<String>,
    job_id:  Option<String>,
    guild_id:  Option<String>,
    channel_id:  Option<String>
}

impl PlayerObject {
    pub async fn new(producer: &'static Producer) -> Self {
        PlayerObject {
            producer: producer,
            worker_id: None,
            job_id: None,
            guild_id: None,
            channel_id: None,
        }
    }
}

pub async fn init_charcoal(broker: String) -> Producer  {
    let brokers = vec![broker];
    let producer : Producer = initialize_producer(initialize_client(&brokers));
    return producer;
}
