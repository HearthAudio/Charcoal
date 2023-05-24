use std::sync::{Arc, Mutex};
use std::time::Duration;
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
use nanoid::nanoid;
use crate::{CONSUMER, PlayerObject, PRODUCER};
use async_trait::async_trait;
use hearth_interconnect::errors::ErrorReport;
use tokio::time::sleep;
use crate::background::processor::IPCData;
use snafu::prelude::*;
use tokio::sync::broadcast::error::SendError;
use crate::background::connector::{boilerplate_parse_ipc, BoilerplateParseIPCError};

#[derive(Debug, Snafu)]
pub enum CreateJobError {
    #[snafu(display("Did not receive job creation confirmation within time-frame"))]
    TimedOutWaitingForJobCreationConfirmation { source: BoilerplateParseIPCError },
    #[snafu(display("Failed to send internal IPC job creation request"))]
    FailedToSendRequest { source: SendError<IPCData> }
}

/// Provides basic functionality to create a job on the hearth server, join a channel, and exit a channel
#[async_trait]
pub trait ChannelManager {
    async fn create_job(&mut self) -> Result<(), CreateJobError>;
    async fn join_channel(&mut self, voice_channel_id: String);
    async fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    /// Create job on Hearth server for this PlayerObject
    async fn create_job(&mut self) -> Result<(),CreateJobError> {
        self.bg_com_tx.send(IPCData::new_from_main(Message::ExternalQueueJob(JobRequest {
            request_id: nanoid!(),
            guild_id: self.guild_id.clone(),
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendRequestSnafu)?;

        //IMPORTANT: This is fine for serenity because each command run's in it's own thread
        //IMPORTANT: But if we add bindings to TS and use Discord.JS im not sure if that would be true. I don't think it would be
        //IMPORTANT: So this would need to change if we did that or it could block all commands
        boilerplate_parse_ipc(|msg| {
            if let IPCData::FromBackground(bg) = msg {
                if let Message::ExternalQueueJobResponse(q) = bg.message {
                    self.job_id = Some(q.job_id);
                    self.worker_id = Some(q.worker_id);
                    return false;
                }
            }
            return true;
        },self.tx.subscribe(),Duration::from_secs(3)).await.context(TimedOutWaitingForJobCreationConfirmationSnafu)?;

        Ok(())

    }
    /// Join Voice Channel
    async fn join_channel(&mut self, voice_channel_id: String) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            guild_id: self.guild_id.clone(),
            voice_channel_id: Some(voice_channel_id),
            play_audio_url: None,
            action_type: DWCActionType::JoinChannel,
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
        }), self.tx.clone(),self.guild_id.clone())).unwrap();

    }
    /// Exit voice channel
    async fn exit_channel(&self) {
        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LeaveChannel,
            play_audio_url: None,
            guild_id: self.guild_id.clone(),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None,
        }), self.tx.clone(),self.guild_id.clone())).unwrap();

    }
}

