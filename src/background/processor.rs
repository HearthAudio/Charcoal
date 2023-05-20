use hearth_interconnect::messages::Message;
use kafka::consumer::Consumer;
use log::{debug, error};
use tokio::sync::broadcast::{Receiver, Sender};

pub async fn init_processor(mut rx: Receiver<Message>, mut tx: Sender<Message>, mut consumer: Consumer) {

    loop {
        let mss = consumer.poll().unwrap();
        if mss.is_empty() {
            debug!("No messages available right now.");
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                let parsed_message : Result<Message,serde_json::Error> = serde_json::from_slice(&m.value);
                match parsed_message {
                    Ok(message) => {
                        match message {
                            Message::ExternalQueueJobResponse(r) => {
                                println!("GOT: {:?} From Kafka",r);
                            },
                            _ => {}
                        }
                    },
                    Err(e) => error!("{} - Failed to parse message",e),
                }
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();

        let rx_data = rx.try_recv();
        match rx_data {
            Ok(d) => {
                println!("{:?}",d);
            },
            Err(e) => {
                error!("{}",e);
            }
        }
    }
}