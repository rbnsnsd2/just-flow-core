// use crate::core::engine;
use pyo3::prelude::*;

mod core;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn process(config: String, flow: String) -> PyResult<String> {
    Ok(core::engine::process(config, flow))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn just_flow(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;

    Ok(())
}

// cargo rustc --lib --release -- -C prefer-dynamic
// strip target/release/libpython_test.so
