// Internal connector




use std::ops::Sub;
use std::process;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use hearth_interconnect::messages::{Message};
use kafka;
use kafka::consumer::Consumer;
use kafka::producer::{Producer, Record, RequiredAcks};
use log::{debug, error, info, warn};
use snafu::prelude::*;
use openssl;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::broadcast::Receiver;
use tokio::time::sleep;
use crate::background::processor::IPCData;
use crate::CharcoalConfig;
use crate::helpers::get_unix_timestamp;
use self::kafka::client::{FetchOffset, KafkaClient, SecurityConfig};
use self::openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};

pub fn initialize_client(brokers: &Vec<String>,config: &CharcoalConfig) -> KafkaClient {
    let mut client : KafkaClient;

    if config.ssl.is_some() {
        // ~ OpenSSL offers a variety of complex configurations. Here is an example:
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_cipher_list("DEFAULT").unwrap();
        builder.set_verify(SslVerifyMode::PEER);

        let cert_file = config.ssl.as_ref().unwrap().ssl_cert.clone();
        let cert_key = config.ssl.as_ref().unwrap().ssl_key.clone();
        let ca_cert = config.ssl.as_ref().unwrap().ssl_ca.clone();

        info!("loading cert-file={}, key-file={}", cert_file, cert_key);

        builder
            .set_certificate_file(cert_file, SslFiletype::PEM)
            .unwrap();
        builder
            .set_private_key_file(cert_key, SslFiletype::PEM)
            .unwrap();
        builder.check_private_key().unwrap();

        builder.set_ca_file(ca_cert).unwrap();

        let connector = builder.build();

        // ~ instantiate KafkaClient with the previous OpenSSL setup
        client = KafkaClient::new_secure(
            brokers.to_owned(),
            SecurityConfig::new(connector)
        );
    } else {
        client = KafkaClient::new(brokers.to_owned());
    }

    // ~ communicate with the brokers
    match client.load_metadata_all() {
        Err(e) => {
            error!("{:?}", e);
            drop(client);
            process::exit(1);
        }
        Ok(_) => {}
    }
    return client;
}

pub fn initialize_producer(client: KafkaClient) -> Producer {
    let producer = Producer::from_client(client)
        // ~ give the brokers one second time to ack the message
        .with_ack_timeout(Duration::from_secs(1))
        // ~ require only one broker to ack the message
        .with_required_acks(RequiredAcks::One)
        // ~ build the producer with the above settings
        .create().unwrap();
    return producer;
}

pub fn send_message(message: &Message, topic: &str, producer: &mut Producer) {
    // Send message to worker
    let data = serde_json::to_string(message).unwrap();
    producer.send(&Record::from_value(topic, data)).unwrap();
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