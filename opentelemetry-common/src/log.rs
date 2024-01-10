//! Logging module.
//!
//! Credit to https://github.com/vincenzopalazzo/nakamoto/blob/master/node/src/logger.rs
use std::str::FromStr;

use log::Level;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::export::logs::LogExporter;
use opentelemetry_sdk::logs::{BatchLogProcessor, LoggerProvider};
use opentelemetry_sdk::runtime;

/// Initialize a new logger.
pub fn init<E: LogExporter + Clone + 'static>(level: &str, exporter: E) -> anyhow::Result<()> {
    let level = Level::from_str(level).map_err(|err| anyhow::anyhow!("{err}"))?;

    //Create a LoggerProvider and register the exporter
    let logger_provider = LoggerProvider::builder()
        .with_log_processor(BatchLogProcessor::builder(exporter.clone(), runtime::Tokio).build())
        .build();

    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).map_err(|err| anyhow::anyhow!("{err}"))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}
