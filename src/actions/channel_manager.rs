
use std::time::Duration;
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};

use nanoid::nanoid;
use crate::{PlayerObject};
use async_trait::async_trait;


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

#[derive(Debug, Snafu)]
pub enum ChannelManagerError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError<IPCData> },
}

/// Provides basic functionality to create a job on the hearth server, join a channel, and exit a channel
#[async_trait]
pub trait ChannelManager {
    async fn create_job(&mut self) -> Result<(), CreateJobError>;
    async fn join_channel(&mut self, voice_channel_id: String) -> Result<(),ChannelManagerError>;
    async fn exit_channel(&self) -> Result<(),ChannelManagerError>;
}

#[async_trait]
impl ChannelManager for PlayerObject {
    /// Create job on Hearth server for this PlayerObject
    async fn create_job(&mut self) -> Result<(),CreateJobError> {

        let guild_id = self.guild_id.clone();

        let tx = self.tx.clone();
        let bg_com = self.bg_com_tx.clone();

        // Does this cause interior mutability? Might be an issue
        let mut job_id =  self.job_id.write().await.clone().unwrap();
        let mut worker_id = self.worker_id.write().await.clone().unwrap();

        tokio::spawn(async move {
            bg_com.send(IPCData::new_from_main(Message::ExternalQueueJob(JobRequest {
                request_id: nanoid!(),
                guild_id: guild_id.clone(),
            }), tx.clone(), guild_id.clone())).unwrap();


            boilerplate_parse_ipc(|msg| {
                if let IPCData::FromBackground(bg) = msg {
                    if let Message::ExternalQueueJobResponse(q) = bg.message {
                        job_id = q.job_id;
                        worker_id = q.worker_id;
                        return false;
                    }
                }
                return true;
            },tx.subscribe(),Duration::from_secs(3)).await.unwrap();
        });

        Ok(())

    }
    /// Join Voice Channel
    async fn join_channel(&mut self, voice_channel_id: String) -> Result<(),ChannelManagerError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.read().await.clone().unwrap(),
            worker_id: self.worker_id.read().await.clone().unwrap(),
            guild_id: self.guild_id.clone(),
            voice_channel_id: Some(voice_channel_id),
            play_audio_url: None,
            action_type: DWCActionType::JoinChannel,
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
        }), self.tx.clone(),self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())

    }
    /// Exit voice channel
    async fn exit_channel(&self) -> Result<(),ChannelManagerError> {
        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.read().await.clone().unwrap(),
            action_type: DWCActionType::LeaveChannel,
            play_audio_url: None,
            guild_id: self.guild_id.clone(),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.read().await.clone().unwrap(),
            voice_channel_id: None,
        }), self.tx.clone(),self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())

    }
}

