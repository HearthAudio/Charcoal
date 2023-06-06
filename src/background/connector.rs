// Internal connector




use std::ops::Sub;
use std::process;

use std::time::{Duration};
use async_fn_traits::AsyncFn4;
use futures::SinkExt;
use hearth_interconnect::messages::{Message};

use log::{error, info};
use nanoid::nanoid;
use snafu::prelude::*;
use openssl;
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::broadcast::Receiver;
use tokio::time::sleep;
use crate::background::processor::IPCData;
use crate::CharcoalConfig;
use crate::helpers::get_unix_timestamp;
use self::openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
fn configure_kafka_ssl(mut kafka_config: ClientConfig,config: &CharcoalConfig) -> ClientConfig {
    if config.ssl.is_some() {
        let ssl = config.ssl.clone().unwrap();
        kafka_config
            .set("security.protocol","ssl")
            .set("ssl.ca.location",ssl.ssl_ca)
            .set("ssl.certificate.location",ssl.ssl_cert)
            .set("ssl.key.location",ssl.ssl_key);
    } else if config.sasl.is_some() {
        let sasl = config.sasl.clone().unwrap();
        kafka_config
            .set("security.protocol","SASL_SSL")
            .set("sasl.mechanisms","PLAIN")
            .set("sasl.username",sasl.kafka_username)
            .set("sasl.password",sasl.kafka_password);
    }
    return kafka_config;
}

pub fn initialize_producer(broker: &str,config: &CharcoalConfig) -> FutureProducer {


    let mut kafka_config = ClientConfig::new()
        .set("bootstrap.servers", broker)
        .clone();

    kafka_config = configure_kafka_ssl(kafka_config,config);

    let producer : FutureProducer = kafka_config.create().expect("Failed to create Producer");

    producer
}

pub async fn initialize_client(brokers: &String, config: &CharcoalConfig) -> StreamConsumer {

    let mut kafka_config = ClientConfig::new()
        .set("group.id", nanoid!())
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol","ssl")
        .clone();

    kafka_config = configure_kafka_ssl(kafka_config,config);

    let consumer : StreamConsumer = kafka_config.create().expect("Failed to create Consumer");

    consumer
        .subscribe(&[&config.kafka_topic])
        .expect("Can't subscribe to specified topic");

    consumer


    
}

pub async fn send_message(message: &Message, topic: &str, producer: &mut FutureProducer) {
    // Send message to worker
    let data = serde_json::to_string(message).unwrap();
    let record : FutureRecord<String,String> = FutureRecord::to(topic).payload(&data);
    producer.send(record, Duration::from_secs(1)).await.unwrap();
}


#[derive(Debug, Snafu)]
pub enum BoilerplateParseIPCError {
    #[snafu(display("Did not receive requested IPC message within specified timeframe"))]
    TimedOutWaitingForIPC { },
}


pub async fn boilerplate_parse_ipc<T>(mut ipc_parser: T, mut rx: Receiver<IPCData>,timeout: Duration) -> Result<(), BoilerplateParseIPCError> where
    T: FnMut(IPCData) -> bool
{
    let start_time = get_unix_timestamp();
    let mut run = true;
    while run {
        let rxm = rx.try_recv();
        match rxm {
            Ok(m) => {
                run = ipc_parser(m);
            },
            Err(e) => {
                match e {
                    TryRecvError::Empty => { }
                    _ => {
                        error!("{}",e);
                    }
                }
            }
        }

        // Handle timeouts
        let current_time = get_unix_timestamp();
        ensure!(current_time.sub(start_time).as_millis() < timeout.as_millis(),TimedOutWaitingForIPCSnafu);
        // Don't max out the CPU
        sleep(Duration::from_millis(150)).await;
    }
    Ok(())
}