use std::sync::Arc;
use hearth_interconnect::messages::{JobRequest, Message};
use hearth_interconnect::worker_communication::{DirectWorkerCommunication, DWCActionType};
use log::{debug, error};
use nanoid::nanoid;
use crate::{InfrastructureType, InternalIPC, InternalIPCType, PlayerObject, StandardActionType};
use async_trait::async_trait;
use crate::background::processor::{ExitChannel, IPCData, JoinChannel};
use crate::connector::{ boilerplate_parse_result, send_message};

#[async_trait]
pub trait ChannelManager {
    async fn join_channel(&mut self,guild_id: String,voice_channel_id: String);
    async fn exit_channel(&self);
}

#[async_trait]
impl ChannelManager for PlayerObject {
    async fn join_channel(&mut self, guild_id: String, voice_channel_id: String) {
        self.tx.send(IPCData::JoinChannel(JoinChannel {
            channel_id: voice_channel_id,
            guild_id,
        })).unwrap();
    }
    async fn exit_channel(&self) {
        self.tx.send(IPCData::ExitChannel(ExitChannel {
            guild_id: self.guild_id.clone().unwrap(),
            job_id: self.job_id.clone().unwrap(),
            worker_id: self.worker_id.clone().unwrap(),
        })).unwrap();
    }
}