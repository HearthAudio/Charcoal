use std::sync::Arc;
use async_trait::async_trait;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::error;
use crate::{PlayerObject, InternalIPC, InternalIPCType};
use crate::actions::helpers::send_direct_worker_communication;
#[async_trait]
pub trait Player {
    async fn play_from_http(&mut self,url: String);
    async fn play_from_youtube(&mut self,url: String);
}

#[async_trait]
impl Player for PlayerObject {
    async fn play_from_http(&mut self, url: String) {
        println!("LOCKING PLAY");
        let mut charcoal = self.charcoal.lock().await;
        println!("SENDING DWC P");
        send_direct_worker_communication(&mut charcoal.producer,DirectWorkerCommunication {
                job_id: self.job_id.clone().unwrap(),
                action_type: DWCActionType::PlayDirectLink,
                play_audio_url: Some(url),
                guild_id: Some(self.guild_id.clone().unwrap()),
                request_id: None,
                new_volume: None,
                seek_position: None,
                loop_times: None,
        },self).await;
        println!("SENT PLAY!");
    }
    async fn play_from_youtube(&mut self,url: String) {
        let mut charcoal = self.charcoal.lock().await;
        send_direct_worker_communication(&mut charcoal.producer,DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::PlayFromYoutube,
            play_audio_url: Some(url),
            guild_id: Some(self.guild_id.clone().unwrap()),
            request_id: None,
            new_volume: None,
            seek_position: None,
            loop_times: None,
        },self).await;
    }
}