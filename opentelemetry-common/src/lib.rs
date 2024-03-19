pub mod log;
pub use anyhow;

use opentelemetry::global;
use opentelemetry_sdk::logs as sdklogs;

pub struct Opentelemetry {
    pub(crate) logger: Option<sdklogs::Logger>,
}

impl Default for Opentelemetry {
    fn default() -> Self {
        Self::new()
    }
}

impl Opentelemetry {
    pub fn new() -> Self {
        Opentelemetry { logger: None }
    }

    pub fn init_log(
        &mut self,
        tag: &str,
        level: &str,
        exporter_endpoint: &str,
    ) -> anyhow::Result<()> {
        log::init(self, tag.to_owned(), level, exporter_endpoint)?;
        Ok(())
    }
}

impl Drop for Opentelemetry {
    fn drop(&mut self) {
        global::shutdown_logger_provider();
    }
}
