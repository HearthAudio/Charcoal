use tokio::sync::broadcast::{Receiver, Sender};
use crate::background::processor::{IPCData, processor};

pub mod processor;
mod actions;

pub async fn init_background(tx: Sender<IPCData>, mut rx: Receiver<IPCData>,brokers: Vec<String>) {
    tokio::task::spawn(async move {
        processor(tx,rx,brokers).await;
    });
}