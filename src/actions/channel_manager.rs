use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::error;
use nanoid::nanoid;
use crate::{InternalIPC, InternalIPCType, PlayerObject, StandardActionType};

pub trait ChannelManager {
    fn join_channel(&mut self,guild_id: String,voice_channel_id: String);
    fn exit_channel(&self);
    fn joined_channel_result(&mut self,job_id: String,worker_id: String);
}

impl ChannelManager for PlayerObject {
    fn join_channel(&mut self, guild_id: String, voice_channel_id: String) {
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::StandardAction(StandardActionType::JoinChannel),
            dwc: None,
            worker_id: "".to_string(),
            job_id: "".to_string(),
            queue_job_request: Some(JobRequest {
                guild_id,
                voice_channel_id,
            }),
            request_id: Some(nanoid!()),
            job_result: Some(&mut self)
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn exit_channel(&self) {
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(DWCActionType::LeaveChannel),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: DWCActionType::LeaveChannel,
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
            job_result: None,
            request_id: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
    fn joined_channel_result(&mut self,job_id: String,worker_id: String) {
        self.job_id = Some(job_id);
        self.worker_id = Some(worker_id);
    }
}