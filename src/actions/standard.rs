//! Standard actions that can be called on a PlayerObject

use crate::background::processor::IPCData;
use crate::{PlayerObjectData, PROKIO_RUNTIME};
use hearth_interconnect::errors::ErrorReport;
use hearth_interconnect::messages::Metadata;
use kanal::ReceiveError;
use log::error;
use prokio::time::sleep;
use snafu::{OptionExt, Snafu};
use std::time::Duration;

pub trait CharcoalEventHandler {
    fn handle_error(&self, report: ErrorReport);
    fn handle_metadata_response(&self, metadata: Metadata);
}

#[derive(Debug, Snafu)]
pub enum RegisterEventHandlerError {
    #[snafu(display("Failed to get Prokio runtime for async job"))]
    FailedToGetProkioRuntime,
}

/// Register an error callback that will be called if an error occurs on this PlayerObject
pub async fn register_event_handler(
    instance: &PlayerObjectData,
    event_handler: impl CharcoalEventHandler + Send + 'static,
) -> Result<(), RegisterEventHandlerError> {
    let t_rx = instance.rx.clone();
    let guild_id = instance.guild_id.clone();

    let runtime = PROKIO_RUNTIME
        .get()
        .context(FailedToGetProkioRuntimeSnafu)?;

    runtime.spawn_pinned(move || async move {
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
                }
                Err(e) => match e {
                    ReceiveError::SendClosed => {
                        break; // If the sender is closed the PlayerObject probably got removed and we should shut down this task
                    }
                    _ => {
                        error!("Register Event Handler Task failed with error: {}", e);
                    }
                },
            }
            sleep(Duration::from_millis(250)).await; // Don't max out the CPU
        }
    });
    Ok(())
}
