use pyo3::{prelude::*, exceptions::PyRuntimeError};


#[pyclass]
pub struct AMap(s_map_rs::AMap);

#[pymethods]
impl AMap {
    #[new]
    fn new() -> PyResult<Self> {
        if let map = s_map_rs::AMap::new() 
        {
            Ok(Self(map))
        } else {
            Err(PyRuntimeError::new_err(format!("Failed to create map")))
        }
    }

    #[pyo3(signature = (key))]
    fn get(&self, key: String) -> PyResult<String> {
        self.0.get(key).cloned()
            .map_err(|e| PyRuntimeError::new_err(format!("Nothing at key: {}",e)))
    }

    #[pyo3(signature = (key, value))]
    fn insert(&mut self, key: String, value: String) -> PyResult<()> {
        self.0.insert(key, value);
        Ok(())
    }
}

#[pymodule]
fn s_map(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<AMap>()?;
    Ok(())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    s_map_rs::sum_as_string(a,b).map_err(|e| {PyRuntimeError::new_err(format!("Failed to sum string, error: {}",e))})
}

/// A Python module implemented in Rust.
#[pymodule]
fn ss(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

