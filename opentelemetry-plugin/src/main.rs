mod plugin;

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
    let plugin = plugin::build_plugin()?;
    plugin.start();
    Ok(())
}
