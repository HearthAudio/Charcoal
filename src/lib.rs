use flume::{Receiver, Sender};
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
    dwc: DirectWorkerCommunication,
    worker_id: String,
    job_id: String
}

pub struct Charcoal {
    tx: Sender<InternalIPC>,
    rx: Receiver<InternalIPC>
}

pub fn init_charcoal() -> Charcoal {
    let (tx, rx) : (Sender<InternalIPC>,Receiver<InternalIPC>) = flume::bounded(1);
    init_connector("".to_string(),tx.clone(),rx.clone());
    return Charcoal {
        tx: tx.clone(),
        rx: rx.clone()
    }
}
