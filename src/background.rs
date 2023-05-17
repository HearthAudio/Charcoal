use std::sync::Arc;
use ipc_rpc::IpcRpc;
use tokio::sync::broadcast::{Receiver, Sender};
use crate::background::processor::{IPCData, processor};

pub mod processor;
pub mod actions;

pub async fn init_background(brokers: Vec<String>,key: &String) {

    tokio::task::spawn(async move {
        processor(key, brokers).await;
    });
}