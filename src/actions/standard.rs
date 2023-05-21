use std::sync::Arc;
use async_fn_traits::{AsyncFn2, AsyncFn3};
use hearth_interconnect::errors::ErrorReport;
use log::error;
use serenity::http::Http;
use crate::background::processor::IPCData;
use crate::PlayerObject;

impl PlayerObject {
    pub async fn register_error_callback<A: AsyncFn3<ErrorReport, Arc<Http>, String, Output = ()> + std::marker::Send + 'static + std::marker::Sync + Send + Sync>(&mut self, callback: A,http: Arc<Http>,channel_id: String)
        where <A as AsyncFn3<hearth_interconnect::errors::ErrorReport, Arc<serenity::http::Http>, std::string::String>>::OutputFuture: std::marker::Send
    {
        let mut t_rx = self.tx.subscribe();
        let guild_id = self.guild_id.clone();
        println!("ERROR C REG");
        tokio::spawn(async move {
            loop {
                let x = t_rx.try_recv();
                match x {
                    Ok(d) => {
                        match d {
                            IPCData::ErrorReport(e) => {
                                println!("ERX: {:?}",e.clone());
                                println!("ERXM: {},{}",guild_id,e.guild_id.clone());
                                if guild_id == e.guild_id {
                                    println!("ERX-REG-CALLBACK");
                                    callback(e,http.clone(),channel_id.clone()).await;
                                }
                            },
                            _ => {}
                        }
                    }
                    Err(e) => {
                        if e.to_string() != "channel empty".to_string() {
                            error!("Failed to RECV with error: {:?}",e);
                        }
                    }
                }
            }
        });
    }
}