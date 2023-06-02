//! Standard actions that can be called on a PlayerObject
use std::sync::Arc;
use std::time::Duration;
use async_fn_traits::{AsyncFn3};
use hearth_interconnect::errors::ErrorReport;
use hearth_interconnect::messages::Metadata;
use log::error;
use serenity::http::Http;
use tokio::time::sleep;
use crate::background::processor::IPCData;
use crate::PlayerObject;

pub trait CharcoalEventHandler {
    fn handle_error(&self,report: ErrorReport);
    fn handle_metadata_response(&self,metadata: Metadata);
}

impl PlayerObject {
    /// Register an error callback that will be called if an error occurs on this PlayerObject
    pub async fn register_event_handler(&mut self, event_handler: impl CharcoalEventHandler + Send + 'static)
    {
        let mut t_rx = self.tx.subscribe();
        let guild_id = self.guild_id.clone();
        tokio::spawn(async move {
            loop {
                let x = t_rx.try_recv();
                match x {
                    Ok(d) => {
                        match d {
                            IPCData::ErrorReport(error_report) => {
                                if guild_id == error_report.guild_id {
                                    event_handler.handle_error(error_report);
                                }
                            },
                            IPCData::MetadataResult(metadata) => {
                                if guild_id == metadata.guild_id {
                                    event_handler.handle_metadata_response(metadata);
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
                sleep(Duration::from_millis(250)).await; // Don't max out the CPU
            }
        });
    }
}