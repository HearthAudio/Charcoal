use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use hearth_interconnect::errors::ErrorReport;
use hearth_interconnect::messages::Message;
use log::{debug, error};
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message as KafkaMessage;
use rdkafka::producer::FutureProducer;

use tokio::sync::broadcast::{Receiver, Sender};
use crate::background::connector::send_message;
use crate::CharcoalConfig;

#[derive(Clone,Debug)]
pub struct FromBackgroundData {
    pub message: Message,
}

#[derive(Clone,Debug)]
pub struct FromMainData {
    pub message: Message,
    pub response_tx: Arc<Sender<IPCData>>,
    pub guild_id: String
}

#[derive(Clone,Debug)]
pub enum IPCData {
    FromBackground(FromBackgroundData),
    FromMain(FromMainData),
    ErrorReport(ErrorReport)
}

// Makes things slightly easier
impl IPCData {
    pub fn new_from_main(message: Message, sender: Arc<Sender<IPCData>>, guild_id: String) -> IPCData {
        IPCData::FromMain(FromMainData {
            message,
            response_tx: sender,
            guild_id
        })
    }
    pub fn new_from_background(message: Message) -> IPCData {
        IPCData::FromBackground(FromBackgroundData {
            message,
        })
    }
}

pub async fn parse_message(message: Message, guild_id_to_tx: &mut HashMap<String, Arc<Sender<IPCData>>>,global_tx: &mut Sender<IPCData>) {
    match &message {
        Message::ErrorReport(e) => {
            error!("GOT Error: {:?} From Hearth Server",e);
            let tx = guild_id_to_tx.get_mut(&e.guild_id);
            match tx {
                Some(tx) => {
                    let gt = tx.send(IPCData::ErrorReport(e.clone()));
                    match gt {
                        Ok(_) => {},
                        Err(e) => {
                            error!("Failed to send error report with error: {:?}",e);
                        }
                    }
                },
                None => {
                    error!("Failed to get appropriate sender when attempting to send error report")
                }
            }
        },
        Message::ExternalJobExpired(_je) => {
            let r = global_tx.send(IPCData::new_from_background(message));
            match r {
                Ok(_) => {},
                Err(e) => {
                    error!("Failed to send Kafka message to main thread once received with error: {}!",e)
                }
            }
        },
        Message::WorkerShutdownAlert(_) => {
            let r = global_tx.send(IPCData::new_from_background(message));
            match r {
                Ok(_) => {},
                Err(e) => {
                    error!("Failed to send Kafka message to main thread once received with error: {}!",e)
                }
            }
        }
        Message::ExternalQueueJobResponse(r) => {
            let tx = guild_id_to_tx.get_mut(&r.guild_id);
            match tx {
                Some(tx) => {
                    let r = tx.send(IPCData::new_from_background(message));
                    match r {
                        Ok(_) => {},
                        Err(e) => {
                            error!("Failed to send Kafka message to main thread once received with error: {}!",e)
                        }
                    }
                },
                None => {
                    error!("Failed to send Response from BG Thread!");
                }
            }

        },
        Message::ExternalMetadataResult(metadata) => {
            let tx = guild_id_to_tx.get_mut(&metadata.guild_id);
            match tx {
                Some(tx) => {
                    let r = tx.send(IPCData::new_from_background(message));
                    match r {
                        Ok(_) => {},
                        Err(e) => {
                            error!("Failed to send Kafka message to main thread once received with error: {}!",e)
                        }
                    }
                },
                None => {
                    error!("Failed to send Response from BG Thread!");
                }
            }
        }
        _ => {}

    }
}

pub async fn init_processor(mut rx: Receiver<IPCData>, mut global_tx: Sender<IPCData>, mut consumer: StreamConsumer,mut producer: FutureProducer,config: CharcoalConfig) {
    let mut guild_id_to_tx: HashMap<String,Arc<Sender<IPCData>>> = HashMap::new();
    loop {
        let mss = consumer.recv().await;
        match mss {
            Ok(m) => {
                let payload = m.payload();

                match payload {
                    Some(payload) => {
                        let parsed_message : Result<Message,serde_json::Error> = serde_json::from_slice(payload);

                        match parsed_message {
                            Ok(m) => {
                                parse_message(m,&mut guild_id_to_tx,&mut global_tx).await;
                            },
                            Err(e) => error!("{}",e)
                        }
                    },
                    None => {
                        error!("Received No Payload!");
                    }

                }

            },
            Err(e) => error!("{}",e)
        }
        
    }
}