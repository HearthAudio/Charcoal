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
pub async fn join_channel_wasm(
    instance: &PlayerObjectData,
    voice_channel_id: String,
    create_job: bool,
) -> Result<(), ChannelManagerError> {
    Ok(())
}

/// Exit voice channel
pub async fn exit_channel_wasm(instance: &PlayerObjectData) -> Result<(), ChannelManagerError> {
    Ok(())
}
