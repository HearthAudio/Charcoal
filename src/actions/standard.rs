use std::sync::Arc;
use async_fn_traits::{AsyncFn2};
use hearth_interconnect::errors::ErrorReport;
use log::error;
use serenity::http::Http;
use crate::background::processor::IPCData;
use crate::PlayerObject;

impl PlayerObject {
    pub async fn register_error_callback(&mut self, callback: impl AsyncFn2<ErrorReport,Arc<Http>,Output = ()> + std::marker::Send + 'static,http: Arc<Http>) {
        let mut t_rx = self.tx.subscribe();
        let guild_id = self.guild_id.clone();
        tokio::spawn(async move {
            loop {
                let x = t_rx.recv().await;
                match x {
                    Ok(d) => {
                        match d {
                            IPCData::ErrorReport(e) => {
                                if guild_id == e.guild_id {
                                    callback(e,http.clone());
                                }
                            },
                            _ => {}
                        }
                    }
                    Err(e) => error!("")
                }
            }
        });
    }
}