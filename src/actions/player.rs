use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use crate::{PlayerObject, InternalIPC, InternalIPCType};

trait Player {
    fn play_from_http(&self,url: String);
    fn play_from_youtube(&self,url: String);
}

impl Player for PlayerObject {
    fn play_from_http(&self,url: String) {
        let _ = self.tx.send_async(InternalIPC {
            action: InternalIPCType::DWCAction(DWCActionType::PlayDirectLink),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: DWCActionType::PlayDirectLink,
                play_audio_url: Some(url),
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
    }
    fn play_from_youtube(&self,url: String) {
        let _ = self.tx.send_async(InternalIPC {
            action: InternalIPCType::DWCAction(DWCActionType::PlayFromYoutube),
            dwc: Some(DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: DWCActionType::PlayFromYoutube,
                play_audio_url: Some(url),
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
    }
}