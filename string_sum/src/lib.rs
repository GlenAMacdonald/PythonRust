use pyo3::prelude::*;

fn sum_to_string(a: usize, b: usize) -> String {
    (a+b).to_string()
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok(sum_to_string(a,b))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(sum_to_string(1,2),'3'.to_string());
    }
}
