mod b58;
mod py_derivation_path;
mod py_network;
mod py_public_key;

use b58::create_b58_module;
use bip32::DerivationPath;
use bip32::XPub;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1;
use bitcoin::util::key::PublicKey;
use par_map::ParMap;
use py_derivation_path::PyDerivationPath;
use py_network::PyNetwork;
use py_public_key::PyPublicKey;
use pyo3::prelude::*;
use pyo3::types::PyString;
use std::iter;
use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/version.rs"));

fn xpub_child_to_pk(derivation_path_str: &str, xpub: XPub) -> PyPublicKey {
    let derivation_path: DerivationPath = DerivationPath::from_str(derivation_path_str).unwrap();
    let mut child_xpub = xpub;

    for child_number in derivation_path {
        child_xpub = child_xpub.derive_child(child_number).unwrap();
    }

    let xpub_as_bytes = child_xpub.to_bytes();

    let sec_pubkey: secp256k1::PublicKey =
        secp256k1::PublicKey::from_slice(&xpub_as_bytes).unwrap();

    let bitcoin_pubkey = PublicKey::new(sec_pubkey);
    PyPublicKey(bitcoin_pubkey)
}

#[pyfunction]
fn public_keys_for_xpub_str_and_paths(
    xpub_str: &str,
    child_paths: Vec<String>,
) -> Vec<PyPublicKey> {
    let xpub = XPub::from_str(xpub_str).unwrap();

    child_paths
        .iter()
        .cloned()
        .zip(iter::repeat(xpub))
        .par_packed_map(32, |(child_path, xpub)| xpub_child_to_pk(&child_path, xpub))
        .collect()
}

/// A Python module implemented in Rust.
#[pymodule]
fn pycoin_rs(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_submodule(create_b58_module(py)?)?;

    m.add_class::<PyDerivationPath>()?;

    m.add_function(wrap_pyfunction!(public_keys_for_xpub_str_and_paths, m)?)?;

    m.add("MAINNET", PyNetwork::new(Network::Bitcoin))?;
    m.add("TESTNET", PyNetwork::new(Network::Testnet))?;

    m.add("__version__", PyString::new(py, semver()))?;
    m.add("__sha__", PyString::new(py, sha()))?;
    m.add("__target__", PyString::new(py, target()))?;
    m.add("__now__", PyString::new(py, now()))?;
    Ok(())
}
