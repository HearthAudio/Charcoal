use flume::{Receiver, Sender};
use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use crate::connector::init_connector;

mod connector;
mod actions;

enum StandardActionType {
    JoinChannel,
    ExitChannel
}


enum InternalIPCType {
    DWCAction(DWCActionType),
    StandardAction(StandardActionType)
}

struct InternalIPC {
    action: InternalIPCType,
    dwc: Option<DirectWorkerCommunication>,
    worker_id: String,
    job_id: String,
    queue_job_request: Option<JobRequest>,
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
            rx: self.rx.clone(),
            worker_id: None,
            job_id: None,
            guild_id: None,
            channel_id: None,
        }
    }
}

pub fn init_charcoal() -> Charcoal {
    let (tx, rx) : (Sender<InternalIPC>,Receiver<InternalIPC>) = flume::bounded(1);
    init_connector("".to_string(),tx.clone(),rx.clone());
    return Charcoal {
        tx: tx.clone(),
        rx: rx.clone()
    }
}
