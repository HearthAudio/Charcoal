use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use log::error;
use nanoid::nanoid;
use snafu::Whatever;
use crate::connector::{boilerplate_parse_result, send_message};

struct JoinChannelResult {
    job_id: String,
    worker_id: String
}

async fn join_channel(guild_id: String, voice_channel_id: String,consumer: &mut Consumer,producer: &mut Producer) -> Result<Option<JoinChannelResult>,Whatever> {
    send_message(&Message::ExternalQueueJob(JobRequest {
        guild_id,
        voice_channel_id,
        request_id: nanoid!(),
    }), "communication", producer);
    // Parse result
    let mut result: Option<JoinChannelResult> = None;
    boilerplate_parse_result(|message| {
        match message {
            Message::ErrorReport(error_report) => {
                error!("{} - Error with Job ID: {} and Request ID: {}",error_report.error,error_report.job_id,error_report.request_id);
                return false;
            },
            Message::ExternalQueueJobResponse(res) => {
                result = Some(JoinChannelResult {
                    worker_id:res.worker_id,
                    job_id: res.job_id
                });
                return false;
            },
            _ => {}
        }
        return true;
    },consumer);
    Ok(result)
}
async fn exit_channel(guild_id: String,job_id: String, producer: &mut Producer) {
    send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        job_id,
        action_type: DWCActionType::LeaveChannel,
        play_audio_url: None,
        guild_id: Some(guild_id),
        request_id: None,
        new_volume: None,
        seek_position: None,
        loop_times: None,
    }),"communication",producer);
}