//! Logging module.
use std::str::FromStr;
use std::sync::Arc;

use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::logs;
use opentelemetry_sdk::resource;
use opentelemetry_sdk::runtime;

use crate::Opentelemetry;

/// Initialize a new logger exported with open telemetry.
pub fn init(
    manager: &mut Opentelemetry,
    tag: String,
    level: &str,
    exporter_endpoint: &str,
) -> anyhow::Result<()> {
    let logger = opentelemetry_otlp::new_pipeline()
        .logging()
        .with_log_config(
            logs::Config::default().with_resource(resource::Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                tag.clone(),
            )])),
        )
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_endpoint(exporter_endpoint),
        )
        .install_batch(runtime::Tokio)?;
    manager.logger = Some(Arc::new(logger));

    // the install method set a global provider, that we can use now
    let logger_provider = opentelemetry::global::logger_provider();
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).map_err(|err| anyhow::anyhow!("{err}"))?;
    let level = log::Level::from_str(level)?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}
