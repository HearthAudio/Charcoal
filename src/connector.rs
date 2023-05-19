// Internal connector




use std::process;

use std::time::Duration;
use hearth_interconnect::messages::{Message};
use log::{debug, error, info, warn};

use rdkafka::Message as KafkaMessage;
use rdkafka::{ClientConfig};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use crate::CharcoalConfig;
use crate::constants::KAFKA_SEND_TIMEOUT;


fn configure_kafka_ssl(mut kafka_config: ClientConfig,config: &CharcoalConfig) -> ClientConfig {
    if config.ssl.is_some() {
        kafka_config
            .set("security.protocol","ssl")
            .set("ssl.ca.location",config.ssl.as_ref().unwrap().ssl_ca.clone())
            .set("ssl.certificate.location",config.ssl.as_ref().unwrap().ssl_cert.clone())
            .set("ssl.key.location",config.ssl.as_ref().unwrap().ssl_key.clone());
    }
    return kafka_config;
}

pub fn initialize_producer(brokers: &String,config: &CharcoalConfig) -> FutureProducer {

    let mut kafka_config = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .clone();

    kafka_config = configure_kafka_ssl(kafka_config,config);

    let producer : FutureProducer = kafka_config.create().expect("Failed to create Producer");

    producer
}

pub async fn initialize_consume(brokers: &String,  config: &CharcoalConfig,group_id: &String) -> StreamConsumer {
    let mut kafka_config = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("security.protocol", "ssl")
        .clone();

    kafka_config = configure_kafka_ssl(kafka_config, config);

    let consumer: StreamConsumer = kafka_config.create().expect("Failed to create Consumer");

    consumer
        .subscribe(&[&config.kafka_topic])
        .expect("Can't subscribe to specified topic");

    consumer
}


pub async fn boilerplate_parse_result<T>(mut message_parser: T, consumer: &mut StreamConsumer) where
T: FnMut(Message) -> bool
{
    let mut check_result = true;
    while check_result {
        let mss = consumer.recv().await;

        match mss {
            Ok(m) => {
                let payload = m.payload();

                match payload {
                    Some(payload) => {
                        let parsed_message : Result<Message,serde_json::Error> = serde_json::from_slice(payload);

                        match parsed_message {
                            Ok(m) => {
                                check_result = message_parser(m);
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


pub async fn send_message(message: &Message, topic: &str, producer: &mut FutureProducer) {
    let data = serde_json::to_string(message).unwrap();
    let record : FutureRecord<String,String> = FutureRecord::to(topic).payload(&data);
    producer.send(record, KAFKA_SEND_TIMEOUT).await.unwrap();
}
