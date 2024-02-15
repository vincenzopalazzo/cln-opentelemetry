//! Plugin implementation
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use serde_json::Value;

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
    let plugin = plugin! {
        state: State::new(),
        dynamic: true,
        notification: [
            on_log,
        ],
        methods: [],
        hooks: [],
    };
    Ok(plugin)
}

#[notification(on = "log")]
fn on_log(_: &mut Plugin<State>, _: &Value) {}
