use hearth_interconnect::messages::{Message, MessageType};
use hearth_interconnect::worker_communication::DirectWorkerCommunication;
use kafka::producer::Producer;
use nanoid::nanoid;
use crate::connector::send_message;
use crate::PlayerObject;

pub async fn send_direct_worker_communication(mut producer: &mut Producer, dwc: DirectWorkerCommunication,player: &PlayerObject) {
    send_message(&Message {
        message_type: MessageType::DirectWorkerCommunication,
        analytics: None,
        queue_job_request: None,
        queue_job_internal: None,
        request_id: nanoid!(),
        worker_id: player.worker_id.clone(),
        direct_worker_communication: Some(dwc),
        external_queue_job_response: None,
        job_event: None,
        error_report: None,
    },"communication",producer)
}