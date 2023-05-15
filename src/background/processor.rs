use std::collections::HashMap;
use std::time::Duration;
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::connector::{initialize_client, initialize_producer};

#[derive(Clone,Debug)]
pub struct SetPlaybackVolume {
    pub volume: f32,
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct ForceStopLoop {
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct LoopIndefinitely {
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct LoopXTimes {
    pub times: usize,
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct SeekToPosition {
    pub pos: Duration,
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct ResumePlayback {
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct PausePlayback {
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct GetMetadata {
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct PlayCommand {
    pub url: String,
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub struct JoinChannel {
    pub channel_id: String,
    pub guild_id: String,
}


#[derive(Clone,Debug)]
pub struct ExitChannel {
    pub guild_id: String,
    pub job_id: String,
    pub worker_id: String
}

#[derive(Clone,Debug)]
pub enum IPCData {
    SetPlaybackVolume(SetPlaybackVolume),
    ForceStopLoop(ForceStopLoop),
    LoopIndefinitely(LoopIndefinitely),
    LoopXTimes(LoopXTimes),
    SeekToPosition(SeekToPosition),
    ResumePlayback(ResumePlayback),
    PausePlayback(PausePlayback),
    GetMetadata(GetMetadata),
    PlayFromHttp(PlayCommand),
    PlayFromYoutube(PlayCommand),
    JoinChannel(JoinChannel),
    ExitChannel(ExitChannel)
}

pub async fn processor(tx: Sender<IPCData>, mut rx: Receiver<IPCData>,brokers: Vec<String>) {
    let producer : Producer = initialize_producer(initialize_client(&brokers));
    let mut consumer = Consumer::from_client(initialize_client(&brokers))
        .with_topic(String::from("communication"))
        .create()
        .unwrap();

    while let Ok(msg) = rx.recv().await {
        println!("{:?}",msg);
        match msg {
            IPCData::SetPlaybackVolume(d) => {}
            IPCData::ForceStopLoop(d) => {}
            IPCData::LoopIndefinitely(d) => {}
            IPCData::LoopXTimes(d) => {}
            IPCData::SeekToPosition(d) => {}
            IPCData::ResumePlayback(d) => {}
            IPCData::PausePlayback(d) => {}
            IPCData::GetMetadata(d) => {}
            IPCData::PlayFromHttp(d) => {}
            IPCData::PlayFromYoutube(d) => {}
            IPCData::JoinChannel(d) => {}
            IPCData::ExitChannel(d) => {}
        }
    }
}