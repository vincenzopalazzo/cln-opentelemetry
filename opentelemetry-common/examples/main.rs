use clap::Parser;

use opentelemetry_common::Opentelemetry;

#[derive(Debug, Parser)]
#[clap(name = "opentelemetry.rs")]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub url: String,
    #[clap(short, long, value_parser)]
    pub message: String,
    #[clap(short, long)]
    pub level: String,
}

#[macro_export]
macro_rules! async_run {
    ($rt:expr, $expr:expr) => {{
        $rt.block_on($expr)
    }};
    ($expr:expr) => {{
        let rt = tokio::runtime::Runtime::new().unwrap();
        $crate::async_run!(rt, $expr)
    }};
}

// the async main is not required by our application
// but the opentelemetry app is requiring to be
// in an async context, so we use this
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let url = args.url;

    let mut manager = Opentelemetry::new();
    manager.init_log("example", &args.level, &url)?;

    match args.level.as_str() {
        "info" => log::info!("{}", args.message),
        "debug" => log::debug!("{}", args.message),
        _ => anyhow::bail!("level `{}` not found", args.level),
    }
    Ok(())
}
