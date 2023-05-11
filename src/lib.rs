use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::connector::init_connector;
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
pub enum InternalIPCType {
    DWCAction(DWCActionType),
    StandardAction(StandardActionType)
}

#[derive(Clone,Debug)]
pub struct InternalIPC {
    action: InternalIPCType,
    dwc: Option<DirectWorkerCommunication>,
    worker_id: String,
    job_id: String,
    queue_job_request: Option<JobRequest>
}

pub struct PlayerObject {
    tx: Sender<InternalIPC>,
    rx: Receiver<InternalIPC>,
    worker_id: Option<String>,
    job_id:  Option<String>,
    guild_id:  Option<String>,
    channel_id:  Option<String>
}

pub struct Charcoal {
    tx: Sender<InternalIPC>,
    rx: Receiver<InternalIPC>
}

impl Charcoal {
    pub fn new_player(&self) -> PlayerObject {
        PlayerObject {
            tx: self.tx.clone(),
            rx: self.tx.subscribe(),
            worker_id: None,
            job_id: None,
            guild_id: None,
            channel_id: None,
        }
    }
}

pub fn init_charcoal(broker: String) -> Charcoal {
    let (tx, rx) : (Sender<InternalIPC>,Receiver<InternalIPC>) = broadcast::channel(16);
    let con_tx = tx.clone();
    setup_logger().expect("Failed to Init Logger - Charcoal");
    tokio::task::spawn(async move {
        init_connector(broker,con_tx,rx);
    });
    return Charcoal {
        tx: tx.clone(),
        rx: tx.subscribe()
    }
}
