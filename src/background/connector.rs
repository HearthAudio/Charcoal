// Internal connector




use std::ops::Sub;
use std::process;

use std::time::{Duration};
use async_fn_traits::AsyncFn4;
use futures::SinkExt;
use hearth_interconnect::messages::{Message};

use log::{error, info};
use snafu::prelude::*;
use openssl;
use rdkafka::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::broadcast::Receiver;
use tokio::time::sleep;
use crate::background::processor::IPCData;
use crate::CharcoalConfig;
use crate::helpers::get_unix_timestamp;
use self::openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
fn configure_kafka_ssl(mut kafka_config: ClientConfig,config: &Config) -> ClientConfig {
    if config.kafka.kafka_use_ssl.unwrap_or(false) {
        kafka_config
            .set("security.protocol","ssl")
            .set("ssl.ca.location",config.kafka.kafka_ssl_ca.clone().expect("Kafka CA Not Found"))
            .set("ssl.certificate.location",config.kafka.kafka_ssl_cert.clone().expect("Kafka Cert Not Found"))
            .set("ssl.key.location",config.kafka.kafka_ssl_key.clone().expect("Kafka Key Not Found"));
    } else if config.kafka.kafka_use_sasl.unwrap_or(false) {
        kafka_config
            .set("security.protocol","SASL_SSL")
            .set("sasl.mechanisms","PLAIN")
            .set("sasl.username",config.kafka.kafka_username.as_ref().unwrap())
            .set("sasl.password",config.kafka.kafka_password.as_ref().unwrap());
    }
    return kafka_config;
}

pub fn initialize_producer(brokers: &String,config: &Config) -> FutureProducer {

    let mut kafka_config = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .clone();

    kafka_config = configure_kafka_ssl(kafka_config,config);

    let producer : FutureProducer = kafka_config.create().expect("Failed to create Producer");

    producer
}

pub async fn initialize_consume_generic(brokers: &String, callback: impl AsyncFn4<Message, Config,Arc<Sender<ProcessorIPCData>>,Option<Arc<Songbird>>,Output = Result<()>>, ipc: &mut ProcessorIPC, initialized_callback: impl AsyncFn1<Config, Output = ()>,songbird: Option<Arc<Songbird>>,group_id: &String) {

    let mut kafka_config = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol","ssl")
        .clone();

    kafka_config = configure_kafka_ssl(kafka_config,config);

    let consumer : StreamConsumer = kafka_config.create().expect("Failed to create Consumer");

    consumer
        .subscribe(&[&config.kafka.kafka_topic])
        .expect("Can't subscribe to specified topic");


    
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