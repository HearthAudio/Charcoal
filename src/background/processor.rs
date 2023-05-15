use std::collections::HashMap;
use std::time::Duration;
use hearth_interconnect::messages::{ExternalQueueJobResponse, Message, Metadata};
use kafka::consumer::Consumer;
use kafka::producer::Producer;
use log::error;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::background::actions::channel_manager::{exit_channel, join_channel};
use crate::background::actions::player::{play_from_http, play_from_youtube};
use crate::background::actions::track_manager::{force_stop_loop, get_metadata, loop_indefinitely, loop_x_times, pause_playback, resume_playback, seek_to_position, set_playback_volume};
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
    ExitChannel(ExitChannel),

    InfrastructureMetadataResult(Metadata),
    InfrastructureJoinResult(ExternalQueueJobResponse)
}

pub async fn processor(tx: Sender<IPCData>, mut rx: Receiver<IPCData>,brokers: Vec<String>) {
    let mut producer : Producer = initialize_producer(initialize_client(&brokers));
    let mut consumer = Consumer::from_client(initialize_client(&brokers))
        .with_topic(String::from("communication"))
        .create()
        .unwrap();

    println!("INIT BACKGROUND");
    loop {
        let msg = rx.try_recv();
        match msg {
            Ok(msg) => {
                println!("{:?}",msg);
                match msg {
                    IPCData::SetPlaybackVolume(d) => {
                        set_playback_volume(&mut producer,d.guild_id,d.job_id,d.volume,d.worker_id).await;
                    }
                    IPCData::ForceStopLoop(d) => {
                        force_stop_loop(&mut producer,d.guild_id,d.job_id,d.worker_id).await;
                    }
                    IPCData::LoopIndefinitely(d) => {
                        loop_indefinitely(&mut producer,d.guild_id,d.job_id,d.worker_id).await;
                    }
                    IPCData::LoopXTimes(d) => {
                        loop_x_times(&mut producer,d.guild_id,d.job_id,d.times,d.worker_id).await;
                    }
                    IPCData::SeekToPosition(d) => {
                        seek_to_position(&mut producer,d.guild_id,d.job_id,d.pos,d.worker_id).await;
                    }
                    IPCData::ResumePlayback(d) => {
                        resume_playback(&mut producer,d.guild_id,d.job_id,d.worker_id).await;
                    }
                    IPCData::PausePlayback(d) => {
                        pause_playback(&mut producer,d.guild_id,d.job_id,d.worker_id).await;
                    }
                    IPCData::GetMetadata(d) => {
                        get_metadata(&mut producer,d.guild_id,d.job_id,d.worker_id).await;
                    }
                    IPCData::PlayFromHttp(d) => {
                        play_from_http(&mut producer,d.guild_id,d.job_id,d.url,d.worker_id).await;
                    }
                    IPCData::PlayFromYoutube(d) => {
                        play_from_youtube(&mut producer,d.guild_id,d.job_id,d.url,d.worker_id).await;
                    }
                    IPCData::JoinChannel(d) => {
                        join_channel(d.channel_id,d.guild_id,&mut producer).await;
                    }
                    IPCData::ExitChannel(d) => {
                        exit_channel(d.guild_id,d.job_id,&mut producer,d.worker_id).await;
                    }
                    IPCData::InfrastructureMetadataResult(_) => {}
                    IPCData::InfrastructureJoinResult(_) => {}
                }
            },
            Err(e) => {}
        }
        // Runner
        let mss = consumer.poll().unwrap();
        if mss.is_empty() {
            continue;
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                let parsed_message : Result<Message,serde_json::Error> = serde_json::from_slice(&m.value);
                match parsed_message {
                    Ok(message) => {
                        match message {
                            Message::ErrorReport(error_report) => {
                                error!("{} - Error with Job ID: {} and Request ID: {}",error_report.error,error_report.job_id,error_report.request_id);
                            },
                            Message::ExternalMetadataResult(m) => {
                                tx.send(IPCData::InfrastructureMetadataResult(m)).unwrap();
                            },
                            Message::ExternalQueueJobResponse(j) => {
                                tx.send(IPCData::InfrastructureJoinResult(j)).unwrap();
                            }
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