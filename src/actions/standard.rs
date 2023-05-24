//! Standard actions that can be called on a PlayerObject
use std::sync::Arc;
use std::time::Duration;
use async_fn_traits::{AsyncFn2, AsyncFn3};
use hearth_interconnect::errors::ErrorReport;
use log::error;
use serenity::http::Http;
use tokio::time::sleep;
use crate::background::processor::IPCData;
use crate::PlayerObject;

impl PlayerObject {
    /// Register an error callback that will be called if an error occurs on this PlayerObject
    pub async fn register_error_callback<A: AsyncFn3<ErrorReport, Arc<Http>, String, Output = ()> + std::marker::Send + 'static + std::marker::Sync + Send + Sync>(&mut self, callback: A,http: Arc<Http>,channel_id: String)
        where <A as AsyncFn3<hearth_interconnect::errors::ErrorReport, Arc<serenity::http::Http>, std::string::String>>::OutputFuture: std::marker::Send
    {
        let mut t_rx = self.tx.subscribe();
        let guild_id = self.guild_id.clone();
        tokio::spawn(async move {
            loop {
                let x = t_rx.try_recv();
                match x {
                    Ok(d) => {
                        if let IPCData::ErrorReport(e) = d {
                            if guild_id == e.guild_id {
                                callback(e,http.clone(),channel_id.clone()).await;
                            }
                        }
                    }
                    Err(e) => {
                        if e.to_string() != "channel empty".to_string() {
                            error!("Failed to RECV with error: {:?}",e);
                        }
                    }
                }
                sleep(Duration::from_millis(250)).await; // Don't max out the CPU
            }
        });
    }
}