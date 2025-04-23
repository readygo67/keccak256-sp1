//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use keccak256_lib::keccak256_iterations;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const KECCAK256_ELF: &[u8] = include_elf!("keccak256-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,
    
    #[arg(long, default_value = "100")]
    n: u32,

    #[arg(long, default_value = "000000")]
    input: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.

    println!("input: {}", args.input);
    let _input = hex::decode(args.input.clone()).unwrap();

    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    stdin.write(&_input);

    if args.execute {
        // Execute the program
        let (_output, report) = client.execute(KECCAK256_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let output = _output.to_vec();
        let expected_output = keccak256_iterations(args.n,_input);

        assert_eq!(output, expected_output);
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(KECCAK256_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");

        let path = format!("keccak256_{}_{}.proof", args.n, args.input);
        proof.save(&path).expect("Failed to save proof");

        println!("Successfully verified proof!");
    }
}
