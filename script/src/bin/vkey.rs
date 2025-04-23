use sp1_sdk::{include_elf, HashableKey, Prover, ProverClient};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const KECCAK256_ELF: &[u8] = include_elf!("keccak256-program");

fn main() {
    let prover = ProverClient::builder().cpu().build();
    let (_, vk) = prover.setup(KECCAK256_ELF);
    println!("vk digest:{}", vk.bytes32());
}
