use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DWCActionType, DirectWorkerCommunication};
use std::time::Duration;
use crate::PlayerObject;
use async_trait::async_trait;
use kanal::SendError;
use nanoid::nanoid;
use crate::background::connector::{boilerplate_parse_ipc, BoilerplateParseIPCError};
use crate::background::processor::IPCData;
use snafu::prelude::*;

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

/// Provides basic functionality to create a job on the hearth server, join a channel, and exit a channel
#[async_trait]
pub trait ChannelManager {
    async fn join_channel(
        &mut self,
        voice_channel_id: String,
        create_job: bool,
    ) -> Result<(), CreateJobError>;
    async fn exit_channel(&self) -> Result<(), ChannelManagerError>;
}

#[async_trait]
impl ChannelManager for PlayerObject {
    /// Create job on Hearth server for this PlayerObject
    async fn join_channel(
        &mut self,
        voice_channel_id: String,
        create_job: bool,
    ) -> Result<(), CreateJobError> {
        Ok(())
    }
    /// Exit voice channel
    async fn exit_channel(&self) -> Result<(), ChannelManagerError> {
        Ok(())
    }
}
