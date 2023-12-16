use super::py_network::PyNetwork;
use bitcoin::util::address::Address;
use bitcoin::util::key::PublicKey;
use pyo3::prelude::*;

#[pyclass(name = "Address")]
struct PyAddress(Address);

#[pymethods]
impl PyAddress {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

#[pyclass(name = "PublicKey")]
pub struct PyPublicKey(pub PublicKey);

#[pymethods]
impl PyPublicKey {
    fn to_string(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    fn p2pkh_address(&self, network: PyNetwork) -> PyResult<PyAddress> {
        match Address::p2pkh(&self.0, network.into()) {
            Ok(address) => Ok(PyAddress(address)),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error creating p2pkh address: {:?}",
                err
            ))),
        }
    }
    
    fn p2wpkh_address(&self, network: PyNetwork) -> PyResult<PyAddress> {
        match Address::p2wpkh(&self.0, network.0) {
            Ok(address) => Ok(PyAddress(address)),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error creating p2wpkh address: {:?}",
                err
            ))),
        }
    }

    fn p2shwpkh_address(&self, network: PyNetwork) -> PyResult<PyAddress> {
        match Address::p2shwpkh(&self.0, network.0) {
            Ok(address) => Ok(PyAddress(address)),
            Err(err) => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error creating p2shwpkh address: {:?}",
                err
            ))),
        }
    }
    
}
