
use std::time::Duration;
use async_trait::async_trait;
use hearth_interconnect::messages::{Message, Metadata};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
use nanoid::nanoid;
use crate::{CONSUMER, PlayerObject, PRODUCER};
use crate::background::processor::IPCData;
use crate::connector::{send_message};

#[async_trait]
/// Provides functionality that can be used once you start playing a track such as: looping, pausing, and resuming.
pub trait TrackManager {
    async fn set_playback_volume(&self,playback_volume: f32);
    async fn force_stop_loop(&self);
    async fn loop_indefinitely(&self);
    async fn loop_x_times(&self,times: usize);
    async fn seek_to_position(&self,position: Duration);
    async fn resume_playback(&self);
    async fn pause_playback(&self);
    async fn get_metadata(&mut self) -> Metadata;
}
#[async_trait]
impl TrackManager for PlayerObject {
    async fn set_playback_volume(&self,playback_volume: f32) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::SetPlaybackVolume,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: Some(playback_volume),
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();
        
    }
    async fn force_stop_loop(&self) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::ForceStopLoop,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();

        
    }
    async fn loop_indefinitely(&self) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LoopForever,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();
        
    }
    async fn loop_x_times(&self,times: usize) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::LoopXTimes,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: Some(times.clone()),
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();
        
    }
    async fn seek_to_position(&self,position: Duration) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::SeekToPosition,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: Some(position.as_millis() as u64),
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();
        
    }
    async fn resume_playback(&self) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::ResumePlayback,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();


        
    }
    async fn pause_playback(&self) {

        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::PausePlayback,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();
        
    }
    async fn get_metadata(&mut self) -> Metadata {

        // let mut px = PRODUCER.lock().await;
        // let p = px.as_mut();
        //
        // let mut cx = CONSUMER.lock().await;
        // let c = cx.as_mut();
        //
        // send_message(&Message::DirectWorkerCommunication(DirectWorkerCommunication {
        //     job_id: self.job_id.clone().unwrap(),
        //     action_type: DWCActionType::GetMetaData,
        //     play_audio_url: None,
        //     guild_id: Some(self.guild_id.clone()),
        //     request_id: Some(nanoid!()),
        //     new_volume: None,
        //     seek_position: None,
        //     loop_times: None,
        //     worker_id: self.worker_id.clone().unwrap(),
        //     voice_channel_id: None
        // }),"communication",&mut *p.unwrap());
        // // Parse result
        // let mut result: Option<Metadata> = None;
        // boilerplate_parse_result(|message| {
        //     match message {
        //         Message::ErrorReport(error_report) => {
        //             error!("{} - Error with Job ID: {} and Request ID: {}",error_report.error,error_report.job_id,error_report.request_id);
        //             return false;
        //         },
        //         Message::ExternalMetadataResult(metadata) => {
        //             result = Some(metadata);
        //             return false;
        //         }
        //         _ => {}
        //     }
        //     return true;
        // },&mut *c.unwrap());


        self.bg_com_tx.send(IPCData::new_from_main(Message::DirectWorkerCommunication(DirectWorkerCommunication {
            job_id: self.job_id.clone().unwrap(),
            action_type: DWCActionType::GetMetaData,
            play_audio_url: None,
            guild_id: Some(self.guild_id.clone()),
            request_id: Some(nanoid!()),
            new_volume: None,
            seek_position: None,
            loop_times: None,
            worker_id: self.worker_id.clone().unwrap(),
            voice_channel_id: None
        }), self.tx.clone(), self.guild_id.clone())).unwrap();

        let result : Metadata;
        loop {
            let res = self.rx.try_recv();
            match res {
                Ok(r) => {
                    if let IPCData::FromBackground(bg) = r {
                        if let Message::ExternalMetadataResult(m) = bg.message {
                            result = m;
                            break;
                        }
                    }
                },
                Err(e) => debug!("Failed to receive message with error on main thread QRX: {}",e),
            }
        };
        return result;
    }
}