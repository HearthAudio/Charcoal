use crate::background::connector::{boilerplate_parse_ipc, BoilerplateParseIPCError};
use crate::background::processor::IPCData;
use crate::PlayerObjectData;
use async_trait::async_trait;
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DWCActionType, DirectWorkerCommunication};
use kanal::SendError;
use nanoid::nanoid;
use snafu::prelude::*;
use std::time::Duration;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Snafu)]
pub enum CreateJobError {
    #[snafu(display("Did not receive job creation confirmation within time-frame"))]
    TimedOutWaitingForJobCreationConfirmation { source: BoilerplateParseIPCError },
    #[snafu(display("Failed to send internal IPC job creation request"))]
    FailedToSendIPC { source: SendError },
}

#[derive(Debug, Snafu)]
pub enum ChannelManagerError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError },
}

/// Create job on Hearth server for this PlayerObject
pub async fn join_channel(
    instance: &PlayerObjectData,
    voice_channel_id: String,
    create_job: bool,
) -> Result<(), CreateJobError> {
    let guild_id = instance.guild_id.clone();

    let tx = instance.tx.clone();
    let bg_com = instance.bg_com_tx.clone();

    let worker_id = instance.worker_id.clone();
    let job_id = instance.job_id.clone();

    if !create_job {
        let job_id = job_id.read().await.clone();
        let worker_id = worker_id.read().await.clone();

        if job_id.is_some() && worker_id.is_some() {
            instance
                .bg_com_tx
                .send(IPCData::new_from_main(
                    Message::DirectWorkerCommunication(DirectWorkerCommunication {
                        job_id: job_id.unwrap(),
                        worker_id: worker_id.unwrap(),
                        guild_id: instance.guild_id.clone(),
                        voice_channel_id: Some(voice_channel_id.clone()),
                        play_audio_url: None,
                        action_type: DWCActionType::JoinChannel,
                        request_id: Some(nanoid!()),
                        new_volume: None,
                        seek_position: None,
                        loop_times: None,
                    }),
                    instance.tx.clone(),
                    instance.guild_id.clone(),
                ))
                .context(FailedToSendIPCSnafu)?;
        }
    }
    // If the above fails fallback to trying to create a new job
    let rx = instance.rx.clone();
    instance.runtime.spawn_pinned(move || async move {
        bg_com
            .send(IPCData::new_from_main(
                Message::ExternalQueueJob(JobRequest {
                    request_id: nanoid!(),
                    guild_id: guild_id.clone(),
                }),
                tx.clone(),
                guild_id.clone(),
            ))
            .unwrap();
        //
        //
        let mut job_id_a = job_id.write().await;
        let mut worker_id_a = worker_id.write().await;
        boilerplate_parse_ipc(
            |msg| {
                if let IPCData::FromBackground(bg) = msg {
                    if let Message::ExternalQueueJobResponse(q) = bg.message {
                        *job_id_a = Some(q.job_id);
                        *worker_id_a = Some(q.worker_id);
                        return false;
                    }
                }
                true
            },
            rx,
            Duration::from_secs(3),
        )
        .await
        .unwrap();
        //
        bg_com
            .send(IPCData::new_from_main(
                Message::DirectWorkerCommunication(DirectWorkerCommunication {
                    job_id: job_id_a.clone().unwrap(),
                    worker_id: worker_id_a.clone().unwrap(),
                    guild_id: guild_id.clone(),
                    voice_channel_id: Some(voice_channel_id),
                    play_audio_url: None,
                    action_type: DWCActionType::JoinChannel,
                    request_id: Some(nanoid!()),
                    new_volume: None,
                    seek_position: None,
                    loop_times: None,
                }),
                tx.clone(),
                guild_id,
            ))
            .context(FailedToSendIPCRequestSnafu)
            .unwrap();
    });
    Ok(())
}
/// Exit voice channel
pub async fn exit_channel(instance: &PlayerObjectData) -> Result<(), ChannelManagerError> {
    instance
        .bg_com_tx
        .send(IPCData::new_from_main(
            Message::DirectWorkerCommunication(DirectWorkerCommunication {
                job_id: instance.job_id.read().await.clone().unwrap(),
                action_type: DWCActionType::LeaveChannel,
                play_audio_url: None,
                guild_id: instance.guild_id.clone(),
                request_id: Some(nanoid!()),
                new_volume: None,
                seek_position: None,
                loop_times: None,
                worker_id: instance.worker_id.read().await.clone().unwrap(),
                voice_channel_id: None,
            }),
            instance.tx.clone(),
            instance.guild_id.clone(),
        ))
        .context(FailedToSendIPCRequestSnafu)?;
    Ok(())
}
