//! Standard actions that can be called on a PlayerObject

use std::time::Duration;
use hearth_interconnect::errors::ErrorReport;
use hearth_interconnect::messages::Metadata;
use log::error;
use prokio::time::sleep;
use crate::background::processor::IPCData;
use crate::PlayerObject;

pub trait CharcoalEventHandler {
    fn handle_error(&self, report: ErrorReport);
    fn handle_metadata_response(&self, metadata: Metadata);
}

impl PlayerObject {
    /// Register an error callback that will be called if an error occurs on this PlayerObject
    pub async fn register_event_handler(
        &mut self,
        event_handler: impl CharcoalEventHandler + Send + 'static,
    ) {
        let mut t_rx = self.rx.clone();
        let guild_id = self.guild_id.clone();
        prokio::spawn_local(async move {
            loop {
                let x = t_rx.try_recv();
                match x {
                    Ok(d) => {
                        if d.is_some() {
                            match d.unwrap() {
                                IPCData::ErrorReport(error_report) => {
                                    if guild_id == error_report.guild_id {
                                        event_handler.handle_error(error_report);
                                    }
                                }
                                IPCData::MetadataResult(metadata) => {
                                    if guild_id == metadata.guild_id {
                                        event_handler.handle_metadata_response(metadata);
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                    Err(e) => {
                        if e.to_string() != *"channel empty" {
                            error!("Failed to RECV with error: {:?}", e);
                        }
                    }
                }
                sleep(Duration::from_millis(250)).await; // Don't max out the CPU
            }
        });
    }
}
