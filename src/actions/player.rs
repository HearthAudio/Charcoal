use std::sync::Arc;
use async_trait::async_trait;
use hearth_interconnect::messages::Message;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::error;
use nanoid::nanoid;
use crate::{PlayerObject, InternalIPC, InternalIPCType, PRODUCER};
use crate::connector::{send_message};

#[async_trait]
pub trait Player {
    async fn play_from_http(&mut self,url: String);
    async fn play_from_youtube(&mut self,url: String);
}

#[async_trait]
impl Player for PlayerObject {
    async fn play_from_http(&mut self, url: String) {
        let mut px = PRODUCER.lock().await;
        let p = px.as_mut();

        send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::PlayDirectLink,
            play_audio_url: Some(url),
            guild_id: Some(self.guild_id.clone().unwrap()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), "communication", &mut *p.unwrap());
        
    }
    async fn play_from_youtube(&mut self,url: String) {

        let mut px = PRODUCER.lock().await;
        let p = px.as_mut();

        send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::PlayFromYoutube,
            play_audio_url: Some(url),
            guild_id: Some(self.guild_id.clone().unwrap()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }),"communication",&mut *p.unwrap());
        
    }
}