use std::time::Duration;
use hearth_interconnect::messages::{Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};

use kafka::producer::Producer;

use nanoid::nanoid;

use crate::connector::{send_message};

pub async fn set_playback_volume(producer: &mut Producer,guild_id: String,job_id: String,playback_volume: f32,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        worker_id,
        action_type: DWCActionType::SetPlaybackVolume,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: Some(playback_volume),
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
pub async fn force_stop_loop(producer: &mut Producer,guild_id: String,job_id: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::ForceStopLoop,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
        worker_id
    }),"communication",producer);

}
pub async fn loop_indefinitely(producer: &mut Producer,guild_id: String,job_id: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::LoopForever,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
        worker_id
    }),"communication",producer);

}
pub async fn loop_x_times(producer: &mut Producer,guild_id: String,job_id: String,times: usize,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::LoopXTimes,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: Some(times.clone()),
        worker_id
    }),"communication",producer);

}
pub async fn seek_to_position(producer: &mut Producer,guild_id: String,job_id: String,position: Duration,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::SeekToPosition,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: Some(position.as_millis() as u64),
        loop_times: None,
        worker_id
    }),"communication",producer);

}
pub async fn resume_playback(producer: &mut Producer,guild_id: String,job_id: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::ResumePlayback,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
        worker_id
    }),"communication",producer);

}
pub async fn pause_playback(producer: &mut Producer,guild_id: String,job_id: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::PausePlayback,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
        worker_id
    }),"communication",producer);

}
pub async fn get_metadata(producer: &mut Producer,guild_id: String,job_id: String,worker_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        worker_id,
        action_type: DWCActionType::GetMetaData,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: Some(nanoid!()),
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);
}