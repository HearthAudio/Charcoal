




use crate::{PlayerObject};
use async_trait::async_trait;
use crate::background::processor::{IPCData, PlayCommand};


#[async_trait]
pub trait Player {
    async fn play_from_http(&mut self,url: String);
    async fn play_from_youtube(&mut self,url: String);
}

#[async_trait]
impl Player for PlayerObject {
    async fn play_from_http(&mut self, url: String) {
        self.tx.send(IPCData::PlayFromHttp(PlayCommand {
            url,
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
        
    }
    async fn play_from_youtube(&mut self,url: String) {
        self.tx.send(IPCData::PlayFromYoutube(PlayCommand {
            url,
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
    }
}