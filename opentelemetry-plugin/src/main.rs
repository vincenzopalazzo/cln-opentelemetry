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

fn main() -> anyhow::Result<()> {
    let plugin = plugin::build_plugin()?;
    plugin.start();
    Ok(())
}
