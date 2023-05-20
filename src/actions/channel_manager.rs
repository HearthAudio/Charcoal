use std::sync::{Arc, Mutex};
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
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

        self.bg_com_tx.send(IPCData::new_from_main(Message::ExternalQueueJob(JobRequest {
            request_id: nanoid!(),
            guild_id: self.guild_id.clone(),
        }), self.tx.clone(), self.guild_id.clone())).unwrap();

        println!("ST-LOOP RECV");

        let mut t_rx = self.tx.subscribe();
        tokio::task::spawn(async move {
            println!("START");
            loop {
                let res = t_rx.try_recv();
                match res {
                    Ok(r) => {
                        println!("RECV TRX: {:?}",r);
                    },
                    Err(e) => debug!("Failed to receive message with error on main thread QRX: {}",e),
                }
            }
        });

        // let mut t_rx = self.tx.subscribe();

        // tokio::task::spawn(async move {
        //     loop {
        //         let res = t_rx.recv().await;
        //         match res {
        //             Ok(r) => {
        //                 println!("RECV TRX: {:?}",r);
        //                 // if let IPCData::FromBackground(b) = r {
        //                 //     if let Message::ExternalQueueJobResponse(j) = b.message {
        //                 //         twi = j.guild_id; //DANGER - DANGER : Not sure about recreating arcs here might cause issues
        //                 //         t_job_id = Arc::from(j.job_id);
        //                 //     }
        //                 // }
        //             },
        //             Err(e) => error!("Failed to receive message with error on main thread QRX: {}",e),
        //         }
        //     }
        // });



    }
    async fn join_channel(&mut self, voice_channel_id: String) {

        // self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
        //     job_id: self.job_id.clone().unwrap(),
        //     guild_id: Some(self.guild_id.clone()),
        //     voice_channel_id: Some(voice_channel_id),
        //     play_audio_url: None,
        //     action_type: DWCActionType::JoinChannel,
        //     request_id: Some(nanoid!()),
        //     new_volume: None,
        //     seek_position: None,
        //     loop_times: None,
        //     worker_id: self.worker_id.clone().unwrap(),
        // }), self.tx.clone(),self.guild_id.clone())).unwrap();

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