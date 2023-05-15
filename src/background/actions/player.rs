use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use log::error;
use nanoid::nanoid;
use snafu::Whatever;
use crate::connector::{send_message};

pub async fn play_from_http(producer: &mut Producer, guild_id: String,job_id: String, url: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        worker_id,
        action_type: DWCActionType::PlayDirectLink,
        play_audio_url: Some(url),
        guild_id: Some(guild_id),
        request_id: Some(nanoid!()),
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
pub async fn play_from_youtube(producer: &mut Producer, guild_id: String,job_id: String, url: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        worker_id,
        action_type: DWCActionType::PlayFromYoutube,
        play_audio_url: Some(url),
        guild_id: Some(guild_id),
        request_id: Some(nanoid!()),
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}