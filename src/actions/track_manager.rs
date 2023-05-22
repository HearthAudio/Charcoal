
use std::time::Duration;
use async_trait::async_trait;
use hearth_interconnect::messages::{Message, Metadata};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
use nanoid::nanoid;
use tokio::time::sleep;
use crate::{CONSUMER, PlayerObject, PRODUCER};
use crate::background::connector::{boilerplate_parse_ipc, BoilerplateParseIPCError};
use crate::background::processor::IPCData;
use snafu::prelude::*;
use tokio::sync::broadcast::error::SendError;

#[derive(Debug, Snafu)]
pub enum TrackActionError {
    #[snafu(display("Failed to send IPC request to Background thread"))]
    FailedToSendIPCRequest { source: SendError<IPCData> },
    #[snafu(display("Did not receive metadata result within timeout time-frame"))]
    TimedOutWaitingForMetadataResult { source: BoilerplateParseIPCError },
}

#[async_trait]
/// Provides functionality that can be used once you start playing a track such as: looping, pausing, and resuming.
pub trait TrackManager {
    /// Set playback volume
    async fn set_playback_volume(&self,playback_volume: f32) -> Result<(),TrackActionError>;
    /// Stop looping
    async fn force_stop_loop(&self) -> Result<(),TrackActionError>;
    /// Loop forever
    async fn loop_indefinitely(&self) -> Result<(),TrackActionError>;
    /// Loop X amount of times
    async fn loop_x_times(&self,times: usize) -> Result<(),TrackActionError>;
    /// Seek to position on track from start
    async fn seek_to_position(&self,position: Duration) -> Result<(),TrackActionError>;
    /// Resume playback
    async fn resume_playback(&self) -> Result<(),TrackActionError>;
    /// Pause playback
    async fn pause_playback(&self) -> Result<(),TrackActionError>;
    /// Get metadata for track currently being played
    async fn get_metadata(&mut self) -> Result<Metadata,TrackActionError>;
}
#[async_trait]
impl TrackManager for PlayerObject {
    async fn set_playback_volume(&self,playback_volume: f32) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::SetPlaybackVolume,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: Some(playback_volume),
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;
        Ok(())
        
    }
    async fn force_stop_loop(&self) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::ForceStopLoop,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())

        
    }
    async fn loop_indefinitely(&self) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LoopForever,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())
        
    }

    async fn loop_x_times(&self,times: usize) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LoopXTimes,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: Some(times.clone()),
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())
        
    }
    async fn seek_to_position(&self,position: Duration) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::SeekToPosition,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: Some(position.as_millis() as u64),
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())
        
    }
    async fn resume_playback(&self) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::ResumePlayback,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())
        
    }
    async fn pause_playback(&self) -> Result<(),TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::PausePlayback,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        Ok(())
        
    }
    async fn get_metadata(&mut self) -> Result<Metadata,TrackActionError> {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::GetMetaData,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).context(FailedToSendIPCRequestSnafu)?;

        let mut result: Option<Metadata> = None;

        boilerplate_parse_ipc(|msg| {
            if let IPCData::FromBackground(bg) = msg {
                if let Message::ExternalMetadataResult(m) = bg.message {
                    result = Some(m);
                    return false
                }
            }
            return true;
        },self.tx.subscribe(),Duration::from_secs(2)).await.context(TimedOutWaitingForMetadataResultSnafu)?;

        Ok(result.expect("This should never happen because the above parser either errors and returns or creates the metadata"))
    }
}