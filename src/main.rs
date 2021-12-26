// src/main.rs
#[macro_use]
extern crate log;

use std::env;

mod core;
mod testz;

use testz::payloads::values::{CONFIG, FLOWSTATE};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("--START--");

    // stuff happens
    // core::engine::engine_hello();

    let result = crate::core::engine::process(CONFIG.to_string(), FLOWSTATE.to_string());
    println!("\nresult: {}", result);
    println!("\n{}", CONFIG.to_string());
    // stop doing stuff
}
