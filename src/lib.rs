use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
fn b58_encode(decoded: &[u8]) -> String {
    bs58::encode(decoded).into_string()
}

#[pyfunction]
fn b58_decode<'p>(py: Python<'p>, encoded: &str) -> &'p PyBytes {
    let s = bs58::decode(encoded).into_vec().expect("bad");
    PyBytes::new(py, &s)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pycoin_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(b58_encode, m)?)?;
    m.add_function(wrap_pyfunction!(b58_decode, m)?)?;
    Ok(())
}
