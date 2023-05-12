use std::sync::Arc;
use hearth_interconnect::messages::{JobRequest, Message, MessageType};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
use nanoid::nanoid;
use crate::{InfrastructureType, InternalIPC, InternalIPCType, PlayerObject, StandardActionType};
use async_trait::async_trait;
use crate::actions::helpers::send_direct_worker_communication;
use crate::connector::send_message;

#[async_trait]
pub trait ChannelManager {
    async fn join_channel(&mut self,guild_id: String,voice_channel_id: String);
    async fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    async fn join_channel(&mut self, guild_id: String, voice_channel_id: String) {
        let mut producer = self.producer.lock().await;
        let mut consumer = self.consumer.lock().await;
        send_message(&Message {
            message_type: MessageType::DirectWorkerCommunication,
            analytics: None,
            queue_job_request: Some(JobRequest {
                guild_id,
                voice_channel_id,
            }),
            queue_job_internal: None,
            request_id: nanoid!(),
            worker_id: None,
            direct_worker_communication: None,
            external_queue_job_response: None,
            job_event: None,
            error_report: None,
        },"communication",&mut producer);
        //

        loop {
            let mss = consumer.poll().unwrap();
            if mss.is_empty() {
                debug!("No messages available right now.");
            }

            for ms in mss.iter() {
                for m in ms.messages() {
                    let parsed_message : Result<Message,serde_json::Error> = serde_json::from_slice(&m.value);
                    match parsed_message {
                        Ok(message) => {
                            match message.message_type {
                                MessageType::ErrorReport => {
                                    let error_report = message.error_report.unwrap();
                                    error!("{} - Error with Job ID: {} and Request ID: {}",error_report.error,error_report.job_id,error_report.request_id)
                                },
                                MessageType::ExternalQueueJobResponse => {
                                    let res = message.external_queue_job_response.unwrap();
                                    self.worker_id = Some(res.worker_id);
                                    self.job_id = Some(res.job_id);
                                    break;

                                },
                                _ => {}
                            }
                        },
                        Err(e) => error!("{} - Failed to parse message",e),
                    }
                }
                let _ = consumer.consume_messageset(ms);
            }
            consumer.commit_consumed().unwrap();
        }
    }
    async fn exit_channel(&self) {
        let mut producer = self.producer.lock().await;
        send_direct_worker_communication(&mut producer,DirectWorkerCommunication {
            job_id: self.job_id.unwrap(),
            action_type: DWCActionType::LeaveChannel,
            play_audio_url: None,
            guild_id: Some(self.guild_id.unwrap()),
            request_id: None,
            new_volume: None,
            seek_position: None,
            loop_times: None,
        });
    }
}