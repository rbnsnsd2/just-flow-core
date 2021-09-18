// src/main.rs
#[macro_use]
extern crate log;
use std::env;

mod core;
mod tests;

use crate::tests::payloads::values::{CONFIG, FLOWSTATE};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("--START--");

    // stuff happens
    core::engine::engine_hello();

    let config: crate::core::structures::Config =
        crate::core::engine::load_config(CONFIG.to_string());
    let input: crate::core::structures::FlowState =
        crate::core::engine::load_flow_state(FLOWSTATE.to_string());
    //
    crate::core::engine::evaluate(config, input);

    // stop doing stuff
}
