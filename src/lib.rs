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
#[pyo3(signature = (
    bound,
    exes=None,
    bound_options=None,
    ignore=None,
    subset=false,
    superset=false,
    user_site=false,
    all_users=false,
    cache_duration=120,
    cache_directory=None,
    log=false
))]
fn validate(
    bound: String,
    exes: Option<Vec<String>>,
    bound_options: Option<Vec<String>>,
    ignore: Option<Vec<String>>,
    subset: bool,
    superset: bool,
    user_site: bool,
    all_users: bool,
    cache_duration: u64,
    cache_directory: Option<String>,
    log: bool,
) -> PyResult<String> {
    if ignore.as_ref().is_some_and(|values| values.is_empty()) {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "ignore cannot be an empty list",
        ));
    }

    let mut child_args = vec!["fetter".to_string()];
    if let Some(exes) = exes {
        for exe in exes {
            child_args.push("-e".to_string());
            child_args.push(exe);
        }
    }
    if cache_duration != 120 {
        child_args.push("--cache-duration".to_string());
        child_args.push(cache_duration.to_string());
    }
    if let Some(cache_directory) = cache_directory {
        child_args.push("--cache-directory".to_string());
        child_args.push(cache_directory);
    }
    if user_site {
        child_args.push("--user-site".to_string());
    }
    if all_users {
        child_args.push("--all-users".to_string());
    }
    if log {
        child_args.push("--log".to_string());
    }

    child_args.push("validate".to_string());
    child_args.push("--bound".to_string());
    child_args.push(bound);
    if let Some(bound_options) = bound_options {
        child_args.push("--bound-options".to_string());
        child_args.extend(bound_options);
    }
    if let Some(ignore) = ignore {
        child_args.push("--ignore".to_string());
        child_args.extend(ignore);
    }
    if subset {
        child_args.push("--subset".to_string());
    }
    if superset {
        child_args.push("--superset".to_string());
    }
    child_args.push("json".to_string());

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
