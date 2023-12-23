/// Formats the sum of two numbers as string.
pub fn sum_as_string(a: usize, b: usize) -> Result<String, String> {
    let c = a + b;
    if c > 10 {
        return Err("Dummy Error, greater than 10".to_string())
    }
    Ok(c.to_string())
}