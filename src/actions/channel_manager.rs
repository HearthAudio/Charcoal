
use hearth_interconnect::messages::{ExternalQueueJobResponse};



use crate::{PlayerObject};
use async_trait::async_trait;
use crate::background::processor::{ExitChannel, IPCData, JoinChannel};

#[async_trait]
pub trait ChannelManager {
    async fn join_channel(&mut self,guild_id: String,voice_channel_id: String) -> ExternalQueueJobResponse;
    async fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    async fn join_channel(&mut self, guild_id: String, voice_channel_id: String) -> ExternalQueueJobResponse {
        self.tx.send(IPCData::JoinChannel(JoinChannel {
            channel_id: voice_channel_id,
            guild_id,
        })).unwrap();
        let mut res : Option<ExternalQueueJobResponse> = None;
        //TODO: Timeout
        println!("STL");
        while let Ok(msg) = self.rx.lock().await.recv().await {
            println!("JOIN WATCH: {:?}",msg);
            match msg {
                IPCData::InfrastructureJoinResult(r) => {
                    res = Some(r)
                }
                _ => {}
            }
        }
        println!("EXIT");
        return res.unwrap();
    }
    async fn exit_channel(&self) {
        self.tx.send(IPCData::ExitChannel(ExitChannel {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
    }
}