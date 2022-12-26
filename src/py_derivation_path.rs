use bip32::DerivationPath;
use pyo3::prelude::*;
use std::str::FromStr;

#[derive(Clone)]
#[pyclass(name = "DerivationPath")]
pub struct PyDerivationPath(pub DerivationPath);

#[pymethods]
impl PyDerivationPath {
    #[staticmethod]
    pub fn parse(s: &str) -> Self {
        Self(DerivationPath::from_str(s).unwrap())
    }
}

impl PyDerivationPath {
    pub fn new(network: DerivationPath) -> Self {
        Self(network)
    }
}
