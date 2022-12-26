use bitcoin::network::constants::Network;
use pyo3::prelude::pyclass;

#[derive(Clone)]
#[pyclass(name = "Network")]
pub struct PyNetwork(Network);

impl PyNetwork {
    pub fn new(network: Network) -> Self {
        Self(network)
    }
}

impl From<PyNetwork> for Network {
    fn from(val: PyNetwork) -> Self {
        val.0
    }
}
