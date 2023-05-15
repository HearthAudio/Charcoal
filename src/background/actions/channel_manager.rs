use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use log::error;
use nanoid::nanoid;
use snafu::Whatever;
use crate::connector::{send_message};



pub async fn join_channel(guild_id: String, voice_channel_id: String,producer: &mut Producer)  {
    send_message(&Message::ExternalQueueJob(JobRequest {
        guild_id,
        voice_channel_id,
        request_id: nanoid!(),
    }), "communication", producer);
}
pub async fn exit_channel(guild_id: String,job_id: String, producer: &mut Producer,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        worker_id,
        action_type: DWCActionType::LeaveChannel,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);
}