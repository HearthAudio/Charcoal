use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::error;
use crate::{InternalIPC, InternalIPCType, PlayerObject};

trait ChannelManager {
    fn join_channel(&self,guild_id: String,voice_channel_id: String);
    fn exit_channel(&self);
}

impl ChannelManager for PlayerObject {
    fn join_channel(&self,guild_id: String,voice_channel_id: String) {
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::DWCAction(DWCActionType::PlayDirectLink),
            dwc: None,
            worker_id: self.worker_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            queue_job_request: Some(JobRequest {
                guild_id,
                voice_channel_id,
            })
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
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
}