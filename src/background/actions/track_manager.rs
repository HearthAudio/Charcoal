use std::time::Duration;
use hearth_interconnect::messages::{JobRequest, Message, Metadata};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use log::error;
use nanoid::nanoid;
use snafu::Whatever;
use crate::connector::{boilerplate_parse_result, send_message};

async fn set_playback_volume(producer: &mut Producer,guild_id: String,job_id: String,playback_volume: f32) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::SetPlaybackVolume,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: Some(playback_volume),
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
async fn force_stop_loop(producer: &mut Producer,guild_id: String,job_id: String,) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::ForceStopLoop,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
async fn loop_indefinitely(producer: &mut Producer,guild_id: String,job_id: String,) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::LoopForever,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
async fn loop_x_times(producer: &mut Producer,guild_id: String,job_id: String,times: usize) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::LoopXTimes,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: Some(times.clone()),
    }),"communication",producer);

}
async fn seek_to_position(producer: &mut Producer,guild_id: String,job_id: String,position: Duration) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::SeekToPosition,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: Some(position.as_millis() as u64),
        loop_times: None,
    }),"communication",producer);

}
async fn resume_playback(producer: &mut Producer,guild_id: String,job_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::ResumePlayback,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
async fn pause_playback(producer: &mut Producer,guild_id: String,job_id: String) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::PausePlayback,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);

}
async fn get_metadata(producer: &mut Producer,guild_id: String,job_id: String,consumer: &mut Consumer) -> Option<Metadata> {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::GetMetaData,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: Some(nanoid!()),
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);
    // Parse result
    let mut result: Option<Metadata> = None;
    boilerplate_parse_result(|message| {
        match message {
            Message::ErrorReport(error_report) => {
                error!("{} - Error with Job ID: {} and Request ID: {}",error_report.error,error_report.job_id,error_report.request_id);
                return false;
            },
            Message::ExternalMetadataResult(metadata) => {
                result = Some(metadata);
                return false;
            }
            _ => {}
        }
        return true;
    }, consumer);
    return result;
}