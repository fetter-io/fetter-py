// use std::env;
use ::fetter::run_cli;
use pyo3::prelude::*;
use pyo3::types::PyList;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn run(args: Vec<String>) -> PyResult<()> {
    // let args: Vec<String> = py_args.iter().map(|arg| arg.to_string()).collect();
    // env::set_var("RUST_BACKTRACE", "1");
    run_cli(args);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn fetter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
