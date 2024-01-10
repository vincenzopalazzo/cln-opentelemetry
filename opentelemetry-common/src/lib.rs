pub mod log;
pub use anyhow;
use opentelemetry_prometheus::ExporterBuilder;
use opentelemetry_sdk::export::logs::LogExporter;

pub enum Exporter {
    PROMETHEUS,
}

pub struct Opentelemetry<E: LogExporter + Clone + 'static> {
    exporter_log: Option<E>,
    exporter_metric: ExporterBuilder,
}

impl<E: LogExporter + Clone + 'static> Opentelemetry<E> {
    pub fn new(exporter_metric: Exporter, exporter_log: E) -> anyhow::Result<Self> {
        let exporter = match exporter_metric {
            Exporter::PROMETHEUS => opentelemetry_prometheus::exporter(),
        };
        Ok(Self {
            exporter_log: None,
            exporter_metric: exporter,
        })
    }
}
