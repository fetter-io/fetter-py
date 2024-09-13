// use std::env;
use ::fetter::run_cli;
use pyo3::prelude::*;
use std::env;

#[pyfunction]
fn run(args: Vec<String>) -> PyResult<()> {
    // let args: Vec<String> = py_args.iter().map(|arg| arg.to_string()).collect();
    // env::set_var("RUST_BACKTRACE", "1");
    run_cli(args);
    Ok(())
}

#[pyfunction]
fn run_with_argv() -> PyResult<()> {
    let args: Vec<String> = env::args().collect();
    run_cli(args);
    Ok(())
}

#[pymodule]
fn fetter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_with_argv, m)?)?;
    Ok(())
}
