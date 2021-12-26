// use crate::core::engine;
use pyo3::prelude::*;

mod core;
mod testz;
use testz::payloads::values::{CONFIG, FLOWSTATE};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn process(config: String, flow: String) -> PyResult<String> {
    Ok(core::engine::process(config, flow))
}

#[pyfunction]
fn example_configuration() -> PyResult<String> {
    Ok(CONFIG.to_string())
}

#[pyfunction]
fn example_flow_state() -> PyResult<String> {
    Ok(FLOWSTATE.to_string())
}

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("Hello".to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn just_flow(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;
    m.add_function(wrap_pyfunction!(example_configuration, m)?)?;
    m.add_function(wrap_pyfunction!(example_flow_state, m)?)?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;

    Ok(())
}

// cargo rustc --lib --release -- -C prefer-dynamic
// strip target/release/libpython_test.so
