
use std::time::Duration;
use async_trait::async_trait;
use hearth_interconnect::messages::{Metadata};



use crate::{PlayerObject};
use crate::background::processor::{ForceStopLoop, GetMetadata, IPCData, LoopIndefinitely, LoopXTimes, PausePlayback, ResumePlayback, SeekToPosition, SetPlaybackVolume};


#[async_trait]
pub trait TrackManager {
    async fn set_playback_volume(&self,playback_volume: f32);
    async fn force_stop_loop(&self);
    async fn loop_indefinitely(&self);
    async fn loop_x_times(&self,times: usize);
    async fn seek_to_position(&self,position: Duration);
    async fn resume_playback(&self);
    async fn pause_playback(&self);
    async fn get_metadata(&self) -> Metadata;
}
#[async_trait]
impl TrackManager for PlayerObject {
    async fn set_playback_volume(&self,playback_volume: f32) {
        self.tx.send(IPCData::SetPlaybackVolume(SetPlaybackVolume {
            volume: playback_volume,
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn force_stop_loop(&self) {
        self.tx.send(IPCData::ForceStopLoop(ForceStopLoop {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn loop_indefinitely(&self) {
        self.tx.send(IPCData::LoopIndefinitely(LoopIndefinitely {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn loop_x_times(&self,times: usize) {
        self.tx.send(IPCData::LoopXTimes(LoopXTimes {
            times: times,
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn seek_to_position(&self,position: Duration) {
        self.tx.send(IPCData::SeekToPosition(SeekToPosition {
            pos: position,
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn resume_playback(&self) {
        self.tx.send(IPCData::ResumePlayback(ResumePlayback {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn pause_playback(&self) {
        self.tx.send(IPCData::PausePlayback(PausePlayback {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn get_metadata(&self) -> Metadata {
        self.tx.send(IPCData::GetMetadata(GetMetadata {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        //
        let mut res : Option<Metadata> = None;
        while let Ok(msg) = self.tx.subscribe().recv().await {
            match msg {
                IPCData::InfrastructureMetadataResult(r) => {
                    res = Some(r)
                }
                _ => {}
            }
        }
        return res.unwrap();
    }
}