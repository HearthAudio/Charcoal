use async_trait::async_trait;
use hearth_interconnect::messages::Message;
use hearth_interconnect::worker_communication::{DWCActionType, DirectWorkerCommunication};
use kanal::SendError;

use crate::background::processor::IPCData;
use crate::{PlayerObjectData, CHARCOAL_INSTANCE};
use nanoid::nanoid;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum PlayerActionError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError },
    #[snafu(display("Failed to get Charcoal Instance"))]
    FailedToGetCharcoalInstance {},
    #[snafu(display("Failed to get Player Instance"))]
    FailedToGetPlayerInstance {},
}

pub async fn play_from_http(guild_id: &str, url: String) -> Result<(), PlayerActionError> {
    let charcoal = CHARCOAL_INSTANCE
        .get()
        .context(FailedToGetCharcoalInstanceSnafu)?;
    let players = charcoal.players.read().await;
    let instance = players
        .get(guild_id)
        .context(FailedToGetPlayerInstanceSnafu)?;
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
pub async fn play_from_youtube(guild_id: &str, url: String) -> Result<(), PlayerActionError> {
    let charcoal = CHARCOAL_INSTANCE
        .get()
        .context(FailedToGetCharcoalInstanceSnafu)?;
    let players = charcoal.players.read().await;
    let instance = players
        .get(guild_id)
        .context(FailedToGetPlayerInstanceSnafu)?;
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
