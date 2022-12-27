// build.rs
use vergen::vergen;

fn main() {
    vergen(vergen::OutputFns::all()).unwrap();
}
