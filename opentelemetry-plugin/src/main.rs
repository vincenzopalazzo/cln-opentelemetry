mod plugin;

fn main() -> anyhow::Result<()> {
    let plugin = plugin::build_plugin()?;
    plugin.start();
    Ok(())
}
