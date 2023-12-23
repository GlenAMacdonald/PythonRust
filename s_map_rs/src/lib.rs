use std::collections::HashMap;
use anyhow::Result;

pub struct AMap {
    map: HashMap<String, String>
}

impl AMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("1".to_string(), "value".to_string());
        Self { map: map }
    }

    pub fn get(&self, key: String) -> Result<&String> {
        match self.map.get(&key) {
            Some(v) => Ok(v),
            None => Err(anyhow::anyhow!("{}", key)) 
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> (){
        self.map.insert(key, value);
    }
}

/// Formats the sum of two numbers as string.
pub fn sum_as_string(a: usize, b: usize) -> Result<String, String> {
    let c = a + b;
    if c > 10 {
        return Err("Dummy Error, greater than 10".to_string())
    }
    Ok(c.to_string())
}