use std::sync::Arc;
use hearth_interconnect::messages::JobRequest;
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::error;
use nanoid::nanoid;
use crate::{InfrastructureType, InternalIPC, InternalIPCType, PlayerObject, StandardActionType};
use async_trait::async_trait;

#[async_trait]
pub trait ChannelManager {
    async fn join_channel(&mut self,guild_id: String,voice_channel_id: String);
    fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    async fn join_channel(&mut self, guild_id: String, voice_channel_id: String) {
        let rid = nanoid!();
        let r = self.tx.send(InternalIPC {
            action: InternalIPCType::StandardAction(StandardActionType::JoinChannel),
            dwc: None,
            worker_id: None,
            job_id: None,
            queue_job_request: Some(JobRequest {
                guild_id,
                voice_channel_id,
            }),
            request_id: Some(rid.clone()),
            job_result: None
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
        loop {
            let res = self.rx.recv().await;
            match res {
                Ok(msg) => {
                    println!("{:?}",msg);
                    match msg.action {
                        InternalIPCType::Infrastructure(InfrastructureType::JoinChannelResult) => {
                            if msg.request_id.unwrap() == rid {
                                println!("Got RESULT!");
                                println!("{},{}",msg.job_id.as_ref().unwrap(),msg.worker_id.as_ref().unwrap());
                                self.job_id = msg.job_id;
                                self.worker_id = msg.worker_id;
                                println!("{},{}",self.job_id.as_ref().unwrap(),self.worker_id.as_ref().unwrap());
                                break;
                            }
                        },
                        _ => {}
                    }
                },
                Err(e) => error!("Error: {}",e)
            }
            println!("LOOP")
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
            worker_id:Some( self.worker_id.clone().unwrap()),
            job_id: Some(self.job_id.clone().unwrap()),
            queue_job_request: None,
            job_result: None,
            request_id: None,
        });
        match r {
            Ok(_) => {},
            Err(e) => error!("Error: {}",e)
        }
    }
}