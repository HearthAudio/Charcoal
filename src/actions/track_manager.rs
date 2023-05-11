use std::time::Duration;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::error;
use crate::{InternalIPC, InternalIPCType, PlayerObject};

pub trait TrackManager {
    fn set_playback_volume(&self,playback_volume: f32);
    fn force_stop_loop(&self);
    fn loop_indefinitely(&self);
    fn loop_x_times(&self,times: usize);
    fn seek_to_position(&self,position: Duration);
    fn resume_playback(&self);
    fn pause_playback(&self);
}

impl TrackManager for PlayerObject {
    fn set_playback_volume(&self,playback_volume: f32) {
        let action_type = DWCActionType::SetPlaybackVolume;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: Some(playback_volume),
                seek_position: None,
                loop_times: None,
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn force_stop_loop(&self) {
        let action_type = DWCActionType::ForceStopLoop;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: None,
                loop_times: None,
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn loop_indefinitely(&self) {
        let action_type = DWCActionType::LoopForever;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: None,
                loop_times: None,
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn loop_x_times(&self,times: usize) {
        let action_type = DWCActionType::LoopXTimes;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: None,
                loop_times: Some(times.clone()),
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn seek_to_position(&self,position: Duration) {
        let action_type = DWCActionType::SeekToPosition;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: Some(position.as_millis() as u64),
                loop_times: None,
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn resume_playback(&self) {
        let action_type = DWCActionType::ResumePlayback;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: None,
                loop_times: None,
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn pause_playback(&self) {
        let action_type = DWCActionType::PausePlayback;
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(action_type.clone()),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: action_type.clone(),
                play_audio_url: None,
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: None,
                loop_times: None,
            }),
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
}