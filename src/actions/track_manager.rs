use crate::background::connector::BoilerplateParseIPCError;
use crate::background::processor::IPCData;
use crate::{PlayerObjectData, CHARCOAL_INSTANCE};
use async_trait::async_trait;
use hearth_interconnect::messages::Message;
use hearth_interconnect::worker_communication::{DWCActionType, DirectWorkerCommunication};
use kanal::SendError;
use nanoid::nanoid;
use snafu::prelude::*;
use std::time::Duration;

#[derive(Debug, Snafu)]
pub enum TrackActionError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError },
    #[snafu(display("Did not receive metadata result within timeout time-frame"))]
    TimedOutWaitingForMetadataResult { source: BoilerplateParseIPCError },
    #[snafu(display("Failed to get Charcoal Instance"))]
    FailedToGetCharcoalInstance {},
    #[snafu(display("Failed to get Player Instance"))]
    FailedToGetPlayerInstance {},
}

pub async fn set_playback_volume(
    playback_volume: f32,
    guild_id: &str,
) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::SetPlaybackVolume,
                play_audio_url: None,
                guild_id: instance.guild_id.clone(),
                request_id: Some(nanoid!()),
                new_volume: Some(playback_volume),
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
pub async fn force_stop_loop(guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::ForceStopLoop,
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
pub async fn loop_indefinitely(guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::LoopForever,
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

pub async fn loop_x_times(times: usize, guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::LoopXTimes,
                play_audio_url: None,
                guild_id: instance.guild_id.clone(),
                request_id: Some(nanoid!()),
                new_volume: None,
                seek_position: None,
                loop_times: Some(times),
                worker_id: instance.worker_id.read().await.clone().unwrap(),
                voice_channel_id: None,
            }),
            instance.tx.clone(),
            instance.guild_id.clone(),
        ))
        .context(FailedToSendIPCRequestSnafu)?;

    Ok(())
}
pub async fn seek_to_position(position: Duration, guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::SeekToPosition,
                play_audio_url: None,
                guild_id: instance.guild_id.clone(),
                request_id: Some(nanoid!()),
                new_volume: None,
                seek_position: Some(position.as_millis() as u64),
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
pub async fn resume_playback(guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::ResumePlayback,
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
pub async fn pause_playback(guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::PausePlayback,
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
pub async fn get_metadata(guild_id: &str) -> Result<(), TrackActionError> {
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
                action_type: DWCActionType::GetMetaData,
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
