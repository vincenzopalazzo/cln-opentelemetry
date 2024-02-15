//! Logging module.
use std::str::FromStr;

use opentelemetry::global::logger_provider;
use opentelemetry_appender_log::OpenTelemetryLogBridge;

/// Initialize a new logger exported with open telemetry.
pub fn init(level: &str) -> anyhow::Result<()> {
    let logger_provider = logger_provider();

    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).map_err(|err| anyhow::anyhow!("{err}"))?;
    let level = log::Level::from_str(level)?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}
