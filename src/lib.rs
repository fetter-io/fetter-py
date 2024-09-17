// use std::env;
use ::fetter::run_cli;
use pyo3::prelude::*;
use std::env;

#[pyfunction]
fn run(args: Vec<String>) -> PyResult<()> {
    run_cli(args);
    Ok(())
}

#[pyfunction]
fn run_with_argv() -> PyResult<()> {
    // the first argument is Python, which must be removed
    let args: Vec<String> = env::args().skip(1).collect();
    run_cli(args);
    Ok(())
}

#[pymodule]
fn fetter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_with_argv, m)?)?;
    Ok(())
}
