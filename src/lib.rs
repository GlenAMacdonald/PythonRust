use pyo3::prelude::*;
use std::collections::HashMap;

// Note that pyO3 doesn't like generic types, so lets be explicit
pub struct PersistentMap {
    map: HashMap<String, String>
}

impl PersistentMap {
    fn new() -> PersistentMap {
        PersistentMap {map: HashMap::new()}
    }

    // fn new() -> Result<Self> {
    //     let tempMap = HashMap<String,String>::new();
    //     Ok(Self {
    //         map: tempMap
    //     })
    // }

    fn add1(&mut self){
        self.map.insert("1".to_string(), "Value".to_string());
    }

    fn get(&self, key: String) -> String {
        return self.map.get(&key).unwrap().clone();
    }
}

/// Gets the value from the map
#[pyfunction]
fn get(a: usize) -> PyResult<()> {
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_map_map(_: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<PersistentMap>()?;
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

#[pymodule]
fn string_map(py: Python, m: &PyModule) -> PyResult<()>{
    // m.add_function(wrap_pyfunction!(new,m)?)?;
    m.add_function(wrap_pyfunction!(get,m)?)?;

    // m.add_wrapped(wrap_pyfunction!(string_map_map))?;
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new(){
        let mut a = PersistentMap::new();
        a.add1();
        assert_eq!(a.get("1".to_string()),"Value");
    }
}