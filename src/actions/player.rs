use async_trait::async_trait;
use hearth_interconnect::messages::Message;
use hearth_interconnect::worker_communication::{DWCActionType, DirectWorkerCommunication};

use crate::background::processor::IPCData;
use crate::PlayerObject;
use nanoid::nanoid;
use snafu::prelude::*;
use tokio::sync::broadcast::error::SendError;

#[derive(Debug, Snafu)]
pub enum PlayerActionError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError<IPCData> },
}

#[async_trait]
/// Allows you to start playback using an HttpRequest or from a Youtube URL
pub trait Player {
    /// Play from an HTTP URL
    async fn play_from_http(&mut self, url: String) -> Result<(), PlayerActionError>;
    /// Play from a Youtube URL
    async fn play_from_youtube(&mut self, url: String) -> Result<(), PlayerActionError>;
}

#[async_trait]
impl Player for PlayerObject {
    async fn play_from_http(&mut self, url: String) -> Result<(), PlayerActionError> {
        self.bg_com_tx
            .send(IPCData::new_from_main(
                Message::DirectWorkerCommunication(DirectWorkerCommunication {
                    job_id: self.job_id.read().await.clone().unwrap(),
                    action_type: DWCActionType::PlayDirectLink,
                    play_audio_url: Some(url),
                    guild_id: self.guild_id.clone(),
                    request_id: Some(nanoid!()),
                    new_volume: None,
                    seek_position: None,
                    loop_times: None,
                    worker_id: self.worker_id.clone().read().await.clone().unwrap(),
                    voice_channel_id: None,
                }),
                self.tx.clone(),
                self.guild_id.clone(),
            ))
            .context(FailedToSendIPCRequestSnafu)?;

        Ok(())
    }
    async fn play_from_youtube(&mut self, url: String) -> Result<(), PlayerActionError> {
        self.bg_com_tx
            .send(IPCData::new_from_main(
                Message::DirectWorkerCommunication(DirectWorkerCommunication {
                    job_id: self.job_id.clone().read().await.clone().unwrap(),
                    action_type: DWCActionType::PlayFromYoutube,
                    play_audio_url: Some(url),
                    guild_id: self.guild_id.clone(),
                    request_id: Some(nanoid!()),
                    new_volume: None,
                    seek_position: None,
                    loop_times: None,
                    worker_id: self.worker_id.clone().read().await.clone().unwrap(),
                    voice_channel_id: None,
                }),
                self.tx.clone(),
                self.guild_id.clone(),
            ))
            .context(FailedToSendIPCRequestSnafu)?;

        Ok(())
    }
}
