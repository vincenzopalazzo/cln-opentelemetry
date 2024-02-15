//! Plugin implementation
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use json::Value;
use opentelemetry_common::Opentelemetry;
use serde_json as json;

use clightningrpc_plugin::commands::RPCCommand;
use clightningrpc_plugin::plugin::Plugin;
use clightningrpc_plugin_macros::{notification, plugin};

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
    plugin.on_init(|_| {
        // FIXME: take from plugin the target + endpoint + log level (I think I can get it from the configuration)
        let resp = Opentelemetry::new("test", "")
            .and_then(|val| val.init_log("info"))
            .ok()
            .map(|_| json::json!({ "disable": "Disabling due a open telemetry init error" }));
        // SAFETY: this is always some, is there is not error, is
        // Some(()) otherwise is Some({ disable: ... })
        resp.unwrap()
    });
    Ok(plugin)
}

#[notification(on = "log")]
fn on_log(_: &mut Plugin<State>, _: &Value) {}
