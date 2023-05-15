use tokio::sync::broadcast::{Receiver, Sender};
use crate::background::processor::{IPCData, processor};

pub mod processor;
pub mod actions;

pub async fn init_background(tx: Sender<IPCData>, rx: Receiver<IPCData>,brokers: Vec<String>) {
    tokio::task::spawn(async move {
        processor(tx,rx,brokers).await;
    });
}