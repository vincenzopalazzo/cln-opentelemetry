//! Plugin implementation
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use json::Value;
use serde::Deserialize;
use serde_json as json;

use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin_macros::{notification, plugin};

use opentelemetry_log::Opentelemetry;

#[derive(Clone, Debug)]
pub(crate) struct State {
    manager: Option<Opentelemetry>,
}

impl State {
    pub fn new() -> Self {
        Self { manager: None }
    }
}

pub fn build_plugin() -> anyhow::Result<Plugin<State>> {
    let mut plugin = plugin! {
        state: State::new(),
        dynamic: true,
        notification: [
           on_log,
        ],
        methods: [],
        hooks: [],
    };
    plugin.add_opt(
        "collect-log-url",
        "string",
        None,
        "Specify the log URL of the remote log collector",
        false,
    );
    plugin.add_opt(
        "collect-log-level",
        "string",
        None,
        "Specify the log level that we would like to track down",
        false,
    );
    plugin.add_opt(
        "collect-tag",
        "string",
        None,
        "Specify the log level that we would like to track down",
        false,
    );

    plugin.on_init(|plugin| {
        let Some(log_url) = plugin.get_opt::<String>("collect-log-url") else {
            return json::json!({
                "disable": "`collect-log-url` option not specified",
            });
        };

        let log_level = plugin
            .get_opt::<String>("collect-log-level")
            .unwrap_or("info".to_owned());

        // FIXME: if it is none set it to the alias, cln should provide always one.
        let Some(log_prefix) = plugin.get_opt::<String>("collect-tag") else {
            return json::json!({
                "disable": "`collect-tag` is not specified, please provide one",
            });
        };

        // Run the Tokio runtime
        let mut manager = Opentelemetry::new();
        let resp = manager.init_log(&log_prefix, &log_level, &log_url);
        if let Err(err) = resp {
            return json::json!({
                "disable": format!("{err}"),
            });
        }

        plugin.state.manager = Some(manager);
        log::info!(target: "test", "opentelemetry plugin starting ....");
        json::json!({})
    });
    Ok(plugin)
}

#[derive(Debug, Clone, Deserialize)]
struct OnLogRequest {
    /// The level of the log
    level: String,
    source: String,
    log: String,
}

#[notification(on = "log")]
fn on_log(plugin: &mut Plugin<State>, request: &Value) {
    let request = request.get("log").unwrap();
    let request = json::from_value::<OnLogRequest>(request.clone()).expect("unable to parse json");
    let logstr = format!("{} {}", request.source, request.log);
    match request.level.as_str() {
        "debug" => log::debug!("{logstr}"),
        "info" => log::info!("{logstr}"),
        "unusual" => log::warn!("{logstr}"),
        "broken" => log::error!("{logstr}"),
        _ => {
            panic!("level not supported `{}`", request.level);
        }
    }
}
