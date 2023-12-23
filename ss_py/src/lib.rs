use pyo3::{prelude::*, exceptions::PyRuntimeError};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    ss_rs::sum_as_string(a,b).map_err(|e| {PyRuntimeError::new_err(format!("Failed to sum string, error: {}",e))})
}

/// A Python module implemented in Rust.
#[pymodule]
fn ss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}