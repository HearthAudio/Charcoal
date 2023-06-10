use crate::background::connector::BoilerplateParseIPCError;
use crate::background::processor::IPCData;
use crate::PlayerObjectData;
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
}

pub async fn set_playback_volume(
    instance: &PlayerObjectData,
    playback_volume: f32,
) -> Result<(), TrackActionError> {
    Ok(())
}
pub async fn force_stop_loop(instance: &PlayerObjectData) -> Result<(), TrackActionError> {
    Ok(())
}
pub async fn loop_indefinitely(instance: &PlayerObjectData) -> Result<(), TrackActionError> {
    Ok(())
}

pub async fn loop_x_times(
    instance: &PlayerObjectData,
    times: usize,
) -> Result<(), TrackActionError> {
    Ok(())
}
pub async fn seek_to_position(
    instance: &PlayerObjectData,
    position: Duration,
) -> Result<(), TrackActionError> {
    Ok(())
}
pub async fn resume_playback(instance: &PlayerObjectData) -> Result<(), TrackActionError> {
    Ok(())
}
pub async fn pause_playback(instance: &PlayerObjectData) -> Result<(), TrackActionError> {
    Ok(())
}
pub async fn get_metadata(instance: &PlayerObjectData) -> Result<(), TrackActionError> {
    Ok(())
}
