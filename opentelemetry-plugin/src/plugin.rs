//! Plugin implementation
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use json::Value;
use serde::Deserialize;
use serde_json as json;

use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin_macros::{notification, plugin};

use opentelemetry_common::Opentelemetry;

#[derive(Clone, Debug)]
pub(crate) struct State;

impl State {
    pub fn new() -> Self {
        State
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
        "str",
        None,
        "Specify the log URL of the remote log collector",
        false,
    );
    plugin.add_opt(
        "collect-log-level",
        "str",
        None,
        "Specify the log level that we would like to track down",
        false,
    );
    plugin.on_init(|plugin| {
        let Some(log_url) = plugin.get_opt::<String>("collect-log-url") else {
            return json::json!({
                "disable": "`log-url` option not specified",
            });
        };

        let log_level = plugin
            .get_opt::<String>("collect-log-level")
            .unwrap_or("info".to_owned());
        let resp = Opentelemetry::new("test", &log_url)
            .and_then(|val| val.init_log(&log_level))
            .ok()
            .map(|_| json::json!({ "disable": "Disabling due a open telemetry init error" }));
        // SAFETY: this is always some, is there is not error, is
        // Some(()) otherwise is Some({ disable: ... })
        resp.unwrap()
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
fn on_log(_: &mut Plugin<State>, request: &Value) {
    log::debug!("receiving `log` notification with body `{request}`");
    let inner_request = request;
    let request = json::from_value::<OnLogRequest>(request.clone());
    if let Err(err) = request {
        log::error!("error while decoding the notification request: `{err}`");
        return;
    }
    // SAFETY: this is safe because we check before
    let request = request.unwrap();
    let logstr = format!("{} {}", request.source, request.log);
    match request.level.as_str() {
        "debug" => log::debug!("{logstr}"),
        "info" => log::info!("{logstr}"),
        "warn" => log::info!("logstr"),
        "error" => log::error!("{logstr}"),
        _ => {
            log::error!("level not supported `{}`", request.level);
            panic!("level not supported `{}`", request.level);
        }
    }
}
