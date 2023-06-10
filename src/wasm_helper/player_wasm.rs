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

pub async fn play_from_http(
    instance: &PlayerObjectData,
    url: String,
) -> Result<(), PlayerActionError> {
    Ok(())
}
pub async fn play_from_youtube(
    instance: &PlayerObjectData,
    url: String,
) -> Result<(), PlayerActionError> {
    Ok(())
}
