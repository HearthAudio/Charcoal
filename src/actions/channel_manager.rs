use std::sync::{Arc, Mutex};
use std::time::Duration;
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
use nanoid::nanoid;
use crate::{CONSUMER, PlayerObject, PRODUCER};
use async_trait::async_trait;
use hearth_interconnect::errors::ErrorReport;
use tokio::time::sleep;
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
    /// Create job on Hearth server for this PlayerObject
    async fn create_job(&mut self) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::ExternalQueueJob(JobRequest {
            request_id: nanoid!(),
            guild_id: self.guild_id.clone(),
        }), self.tx.clone(), self.guild_id.clone())).unwrap();


        // This is a bit shit but for some reason if we try and recv().await here instead of try_recv() we dont get any messages
        loop {
            let res = self.rx.try_recv();
            match res {
                Ok(r) => {
                    if let IPCData::FromBackground(bg) = r {
                        if let Message::ExternalQueueJobResponse(q) = bg.message {
                            self.job_id = Some(q.job_id);
                            self.worker_id = Some(q.worker_id);
                            break;
                        }
                    }
                },
                Err(e) => debug!("Failed to receive message with error on main thread QRX: {}",e),
            }
            sleep(Duration::from_millis(100)).await; // Don't max out the CPU
        }


    }
    /// Join Voice Channel
    async fn join_channel(&mut self, voice_channel_id: String) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
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
        }), self.tx.clone(),self.guild_id.clone())).unwrap();

    }
    /// Exit voice channel
    async fn exit_channel(&self) {
        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
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
        }), self.tx.clone(),self.guild_id.clone())).unwrap();

    }
}

