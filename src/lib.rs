use std::sync::{Arc};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::broadcast::{Sender,Receiver};
use tokio::sync::{broadcast, Mutex};
use crate::background::init_background;
use crate::background::processor::IPCData;

mod connector;
pub mod actions;
pub mod serenity;
mod background;

#[derive(Clone,Debug)]
pub struct JobResult {
    pub job_id: String,
    pub worker_id: String
}

pub struct PlayerObject {
    tx: Arc<Sender<IPCData>>,
    rx: Arc<Mutex<Receiver<IPCData>>>,
    worker_id: Option<String>,
    job_id:  Option<String>,
    guild_id:  Option<String>,
    channel_id:  Option<String>
}

impl PlayerObject {
    pub async fn new(charcoal: &mut Charcoal, guild_id: String) -> Self {

        let tx = charcoal.tx.clone();

        tx.send(IPCData::InfrastructureRegisterNewRXPair(guild_id.clone())).unwrap();

        let mut rx: Option<Arc<Mutex<Receiver<IPCData>>>> = None;
        while let Ok(msg) = charcoal.rx.recv().await {
            println!("SANITY: {:?}",msg);
            match msg {
                IPCData::InfrastructureRegisterNewRXPairResult(r) => {
                    rx = Some(r);
                }
                _ => {}
            }
        }
        println!("GOT NEW RX PAIR");

        PlayerObject {
            tx: Arc::new(tx),
            rx: rx.unwrap(),
            worker_id: None,
            job_id: None,
            guild_id: Some(guild_id.clone()),
            channel_id: None,
        }
    }
}

pub struct Charcoal {
    tx: Sender<IPCData>,
    rx: Receiver<IPCData>
}

pub async fn init_charcoal(broker: String) -> Charcoal  {
    let brokers = vec![broker];
    let (tx, rx) = broadcast::channel(16);
    let mut second_rx = tx.subscribe();
    let rxx = tx.subscribe();
    init_background(tx.clone(),rxx,brokers).await;

    // sleep(Duration::from_secs(1));
    // tx.send(IPCData::InfrastructureRegisterNewRXPair("1103499477962207332".to_string())).unwrap();
    // while let Ok(msg) = second_rx.recv().await {
    //     println!("SANITY: {:?}",msg);
    // }


    return Charcoal {
        tx: tx.clone(),
        rx: second_rx
    };
}
