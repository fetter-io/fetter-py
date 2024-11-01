// use std::env;
use ::fetter::run_cli;
use pyo3::prelude::*;
use std::env;

#[pyfunction]
fn run(args: Vec<String>) -> PyResult<()> {
    match run_cli(args) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error: {}", e)))
        }
    }
}


// #[pyfunction]
// fn run_with_argv() -> PyResult<()> {
//     // the first argument is Python, which must be removed
//     let args: Vec<String> = env::args().skip(1).collect();
//     let _ = run_cli(args);
//     Ok(())
// }


#[pyfunction]
fn run_with_argv() -> PyResult<()> {
    // the first argument is Python, which must be removed
    let args: Vec<String> = env::args().skip(1).collect();
    match run_cli(args) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[pymodule]
fn fetter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_with_argv, m)?)?;
    Ok(())
}
