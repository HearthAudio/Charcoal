use async_trait::async_trait;
use hearth_interconnect::messages::Message;
use hearth_interconnect::worker_communication::{DWCActionType, DirectWorkerCommunication};
use kanal::SendError;

use crate::background::processor::IPCData;
use crate::PlayerObjectData;
use nanoid::nanoid;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PlayerActionError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError },
}

async fn play_from_http(instance: &PlayerObjectData, url: String) -> Result<(), PlayerActionError> {
    instance
        .bg_com_tx
        .send(IPCData::new_from_main(
            Message::DirectWorkerCommunication(DirectWorkerCommunication {
                job_id: instance.job_id.read().await.clone().unwrap(),
                action_type: DWCActionType::PlayDirectLink,
                play_audio_url: Some(url),
                guild_id: instance.guild_id.clone(),
                request_id: Some(nanoid!()),
                new_volume: None,
                seek_position: None,
                loop_times: None,
                worker_id: instance.worker_id.clone().read().await.clone().unwrap(),
                voice_channel_id: None,
            }),
            instance.tx.clone(),
            instance.guild_id.clone(),
        ))
        .context(FailedToSendIPCRequestSnafu)?;

    Ok(())
}
async fn play_from_youtube(
    instance: &PlayerObjectData,
    url: String,
) -> Result<(), PlayerActionError> {
    instance
        .bg_com_tx
        .send(IPCData::new_from_main(
            Message::DirectWorkerCommunication(DirectWorkerCommunication {
                job_id: instance.job_id.clone().read().await.clone().unwrap(),
                action_type: DWCActionType::PlayFromYoutube,
                play_audio_url: Some(url),
                guild_id: instance.guild_id.clone(),
                request_id: Some(nanoid!()),
                new_volume: None,
                seek_position: None,
                loop_times: None,
                worker_id: instance.worker_id.clone().read().await.clone().unwrap(),
                voice_channel_id: None,
            }),
            instance.tx.clone(),
            instance.guild_id.clone(),
        ))
        .context(FailedToSendIPCRequestSnafu)?;

    Ok(())
}
