#![no_main]
sp1_zkvm::entrypoint!(main);

// use tiny_keccak::{Hasher, Keccak};
use keccak256_lib::keccak256_iterations;

pub fn main() {
    let n = sp1_zkvm::io::read::<u32>();
    let input = sp1_zkvm::io::read::<Vec<u8>>();
    let output = keccak256_iterations(n, input);

    sp1_zkvm::io::commit(&output);
}
