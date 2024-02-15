use pyo3::prelude::*;

#[pyfunction]
fn inverse_hash(hashstring: &str) -> String {
    hashstring
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(2)
        .flat_map(|chunk| chunk.iter().rev())
        .collect::<String>()
}

/// A Python module implemented in Rust.
pub fn create_utils_module(py: Python) -> PyResult<&'_ PyModule> {
    let m = PyModule::new(py, "utils")?;
    m.add_function(wrap_pyfunction!(inverse_hash, m)?)?;
    Ok(m)
}