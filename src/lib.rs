use ::fetter::run_cli;
use ::fetter::write_color;
use ::fetter::UreqClientLive;
use pyo3::prelude::*;
use std::io::stderr;
use std::process::Command;
use std::sync::Arc;

use std::env;

#[pyfunction]
fn run(args: Vec<String>) -> PyResult<()> {
    let client = Arc::new(UreqClientLive);

    match run_cli(args, client) {
        Ok(_) => Ok(()),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Error: {}",
            e
        ))),
    }
}

#[pyfunction]
fn run_with_argv() -> PyResult<()> {
    // the first argument is Python, which must be removed
    let args: Vec<String> = env::args().skip(1).collect();
    let client = Arc::new(UreqClientLive);

    if let Err(e) = run_cli(args, client) {
        let mut stderr = stderr();
        write_color(&mut stderr, "#666666", "fetter ");
        write_color(&mut stderr, "#cc0000", "Error: ");
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

#[pyfunction]
fn validate(args: Vec<String>) -> PyResult<String> {
    if !args.iter().any(|arg| arg == "validate") {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "validate() expects CLI-style args that include the `validate` command",
        ));
    }
    if args
        .iter()
        .any(|arg| matches!(arg.as_str(), "display" | "write" | "exit"))
    {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "validate() always returns JSON and does not accept validate output subcommands",
        ));
    }

    let mut child_args = args;
    if !child_args.iter().any(|arg| arg == "json") {
        child_args.push("json".to_string());
    }

    let python_executable = Python::attach(|py| -> PyResult<String> {
        let sys = py.import("sys")?;
        sys.getattr("executable")?.extract()
    })?;

    let output = Command::new(python_executable)
        .arg("-c")
        .arg("import sys, fetter; fetter.run(sys.argv[1:])")
        .args(child_args)
        .output()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    if !output.status.success() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[pymodule]
fn fetter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_with_argv, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    Ok(())
}
