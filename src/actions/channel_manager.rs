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
        let guild_id = self.guild_id.clone();

        let tx = self.tx.clone();
        let bg_com = self.bg_com_tx.clone();

        let worker_id = self.worker_id.clone();
        let job_id = self.job_id.clone();

        if !create_job {
            self.bg_com_tx
                .send(IPCData::new_from_main(
                    Message::DirectWorkerCommunication(DirectWorkerCommunication {
                        job_id: job_id.read().unwrap().clone().unwrap(),
                        worker_id: worker_id.read().unwrap().clone().unwrap(),
                        guild_id: self.guild_id.clone(),
                        voice_channel_id: Some(voice_channel_id.clone()),
                        play_audio_url: None,
                        action_type: DWCActionType::JoinChannel,
                        request_id: Some(nanoid!()),
                        new_volume: None,
                        seek_position: None,
                        loop_times: None,
                    }),
                    self.tx.clone(),
                    self.guild_id.clone(),
                ))
                .context(FailedToSendIPCSnafu)?;

            return Ok(());
        }
        let rx = self.rx.clone();
        prokio::spawn_local(async move {
            bg_com
                .send(IPCData::new_from_main(
                    Message::ExternalQueueJob(JobRequest {
                        request_id: nanoid!(),
                        guild_id: guild_id.clone(),
                    }),
                    tx.clone(),
                    guild_id.clone(),
                ))
                .unwrap();
            //
            //
            let mut job_id_a = job_id.write().unwrap();
            let mut worker_id_a = worker_id.write().unwrap();
            boilerplate_parse_ipc(
                |msg| {
                    if let IPCData::FromBackground(bg) = msg {
                        if let Message::ExternalQueueJobResponse(q) = bg.message {
                            *job_id_a = Some(q.job_id);
                            *worker_id_a = Some(q.worker_id);
                            return false;
                        }
                    }
                    true
                },
                rx,
                Duration::from_secs(3),
            )
            .await
            .unwrap();
            //
            bg_com
                .send(IPCData::new_from_main(
                    Message::DirectWorkerCommunication(DirectWorkerCommunication {
                        job_id: job_id_a.clone().unwrap(),
                        worker_id: worker_id_a.clone().unwrap(),
                        guild_id: guild_id.clone(),
                        voice_channel_id: Some(voice_channel_id),
                        play_audio_url: None,
                        action_type: DWCActionType::JoinChannel,
                        request_id: Some(nanoid!()),
                        new_volume: None,
                        seek_position: None,
                        loop_times: None,
                    }),
                    tx.clone(),
                    guild_id,
                ))
                .context(FailedToSendIPCRequestSnafu)
                .unwrap();
        });

        Ok(())
    }
    /// Exit voice channel
    async fn exit_channel(&self) -> Result<(), ChannelManagerError> {
        self.bg_com_tx
            .send(IPCData::new_from_main(
                Message::DirectWorkerCommunication(DirectWorkerCommunication {
                    job_id: self.job_id.read().unwrap().clone().unwrap(),
                    action_type: DWCActionType::LeaveChannel,
                    play_audio_url: None,
                    guild_id: self.guild_id.clone(),
                    request_id: Some(nanoid!()),
                    new_volume: None,
                    seek_position: None,
                    loop_times: None,
                    worker_id: self.worker_id.read().unwrap().clone().unwrap(),
                    voice_channel_id: None,
                }),
                self.tx.clone(),
                self.guild_id.clone(),
            ))
            .context(FailedToSendIPCRequestSnafu)?;

        Ok(())
    }
}
