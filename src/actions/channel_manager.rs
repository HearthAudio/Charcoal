
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{error};
use nanoid::nanoid;
use crate::{CONSUMER, PlayerObject, PRODUCER};
use async_trait::async_trait;
use crate::connector::{ boilerplate_parse_result, send_message};

#[async_trait]
pub trait ChannelManager {
    async fn create_job(&mut self);
    async fn join_channel(&mut self, voice_channel_id: String,guild_id: String);
    async fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    async fn create_job(&mut self) {

        let mut px = PRODUCER.lock().await;
        let p = px.as_mut();

        let mut cx = CONSUMER.lock().await;
        let c = cx.as_mut();

        send_message(&Message::ExternalQueueJob(JobRequest {
            request_id: nanoid!(),
        }), "communication", &mut p.unwrap());
        // Parse result
        boilerplate_parse_result(|message| {
            match message {
                Message::ErrorReport(error_report) => {
                    error!("{} - Error with Job ID: {} and Request ID: {}",error_report.error,error_report.job_id,error_report.request_id);
                    return false;
                },
                Message::ExternalQueueJobResponse(res) => {
                    self.worker_id = Some(res.worker_id);
                    self.job_id = Some(res.job_id);
                    return false;
                },
                _ => {}
            }
            return true;
        },&mut *c.unwrap());
    }
    async fn join_channel(&mut self, voice_channel_id: String,guild_id: String) {
        self.guild_id = Some(guild_id);

        let mut px = PRODUCER.lock().await;
        let p = px.as_mut();

        let mut cx = CONSUMER.lock().await;
        let _c = cx.as_mut();

        send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            guild_id: self.guild_id.clone(),
            voice_channel_id: Some(voice_channel_id),
            play_audio_url: None,
            action_type: DWCActionType::JoinChannel,
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
        }), "communication", &mut p.unwrap());
    }
    async fn exit_channel(&self) {
        let mut px = PRODUCER.lock().await;
        let p = px.as_mut();

        send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LeaveChannel,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone().unwrap()),
            request_id: None,
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None,
        }), "communication", &mut *p.unwrap());
        
    }
}