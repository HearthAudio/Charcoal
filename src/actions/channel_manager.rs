
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{error};
use nanoid::nanoid;
use crate::{CONSUMER, PlayerObject, PRODUCER};
use async_trait::async_trait;
use crate::background::processor::IPCData;
use crate::connector::{send_message};

#[async_trait]
/// Provides basic functionality to create a job on the hearth server, join a channel, and exit a channel
pub trait ChannelManager {
    async fn create_job(&mut self);
    async fn join_channel(&mut self, voice_channel_id: String);
    async fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    async fn create_job(&mut self) {

        self.tx.send(IPCData::new_from_main(Message::ExternalQueueJob(JobRequest {
            request_id: nanoid!(),
        }))).unwrap();

        println!("ST-LOOP RECV");
        loop {
            let res = self.rx.recv().await;
            match res {
                Ok(r) => {
                    println!("RECV: {:?}",r);
                    if r.from_background {
                        if let Message::ExternalQueueJobResponse(j) = r.message {
                            self.worker_id = Some(j.worker_id);
                            self.job_id = Some(j.job_id);
                        }
                    }
                },
                Err(e) => error!("Failed to receive message with error on main thread: {}",e),
            }
        }

    }
    async fn join_channel(&mut self, voice_channel_id: String) {

        self.tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            guild_id: Some(self.guild_id.clone()),
            voice_channel_id: Some(voice_channel_id),
            play_audio_url: None,
            action_type: DWCActionType::JoinChannel,
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
        }))).unwrap();

    }
    async fn exit_channel(&self) {
        let mut px = PRODUCER.lock().await;
        let p = px.as_mut();

        send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LeaveChannel,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None,
        }), "communication", &mut *p.unwrap());
        
    }
}