# SP1 Project Template

This is a template for creating an end-to-end [SP1](https://github.com/succinctlabs/sp1) project
that can generate a proof of any RISC-V program.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/docs/sp1/getting-started/install)

## Running the Project

There are 3 main ways to run this project: execute a program, generate a core proof.

### Build the Program

The program is automatically built through `script/build.rs` when the script is built.

### Execute the Program

The default setting is running with keccak precompile, to run without precompile, change the `tiny-keccak` [dependencies](./Cargo.toml) to:

`tiny-keccak = { version = "2.0.2", features = ["keccak"] }`

To run the program without generating a proof:

```sh
cd script
RUST_LOG=info cargo run --release -- --execute --n 10000 --input "000000" //对0x000000 执行10000次keccak
```

This will execute the program and display the output.

### Generate an SP1 Core Proof

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
RUST_LOG=info cargo run --release -- --prove --n 10000 --input "000000" //对0x000000 执行10000次keccak
```

### Get the Vk's digest

```sh
cd script
RUST_LOG=info cargo run --bin vkey
```
