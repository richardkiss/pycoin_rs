use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

include!(concat!(env!("OUT_DIR"), "/version.rs"));

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
fn pycoin_rs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(b58_encode, m)?)?;
    m.add_function(wrap_pyfunction!(b58_decode, m)?)?;
    m.add(&"__version__", PyString::new(py, &semver()))?;
    m.add(&"__sha__", PyString::new(py, &sha()))?;
    m.add(&"__target__", PyString::new(py, &target()))?;
    m.add(&"__now__", PyString::new(py, &now()))?;
    Ok(())
}
