pub mod log;
pub use anyhow;

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::logs;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::Resource;

pub struct Opentelemetry {
    _logger: logs::Logger,
}

impl Opentelemetry {
    pub fn new(tag: &str, exporter_endpoint: &str) -> anyhow::Result<Self> {
        let logger = opentelemetry_otlp::new_pipeline()
            .logging()
            .with_log_config(logs::Config::default().with_resource(Resource::new(vec![
                KeyValue::new(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    tag.to_owned(),
                ),
            ])))
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(exporter_endpoint),
            )
            .install_batch(runtime::Tokio)?;
        Ok(Self { _logger: logger })
    }

    pub fn init_log(&self, level: &str) -> anyhow::Result<()> {
        log::init(level)?;
        Ok(())
    }
}
