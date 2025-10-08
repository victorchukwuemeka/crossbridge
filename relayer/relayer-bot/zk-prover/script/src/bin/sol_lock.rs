// script/src/bin/sol_lock.rs
use alloy_sol_types::SolType;
use clap::Parser;
use sol_lock_lib::SolLockPublicValues;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin}; // Import from lib

pub const SOL_LOCK_ELF: &[u8] = include_elf!("sol-lock-program");

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "100000000")]
    amount: u64,

    #[arg(long, default_value = "123456")]
    slot: u64,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: Specify either --execute or --prove");
        std::process::exit(1);
    }

    let client = ProverClient::from_env();

    // Create dummy data (same as before)
    let dummy_merkle_root = [1u8; 32];
    let dummy_merkle_proof = vec![[2u8; 32], [3u8; 32]];
    let dummy_leaf_data = [4u8; 32];
    let dummy_leaf_index = 0u32;
    let dummy_sender = [5u8; 32];
    let dummy_receiver = [6u8; 20];

    // Setup inputs using SP1Stdin (following Fibonacci pattern)
    let mut stdin = SP1Stdin::new();
    stdin.write(&dummy_merkle_root);
    stdin.write(&(dummy_merkle_proof.len() as u32));
    for item in &dummy_merkle_proof {
        stdin.write(item);
    }
    stdin.write(&dummy_leaf_data);
    stdin.write(&dummy_leaf_index);
    stdin.write(&dummy_sender);
    stdin.write(&dummy_receiver);
    stdin.write(&args.amount);
    stdin.write(&args.slot);

    println!("🔐 SOL Lock Verification:");
    println!("   Amount: {} lamports", args.amount);
    println!("   Slot: {}", args.slot);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(SOL_LOCK_ELF, &stdin).run().unwrap();
        println!("✅ Program executed successfully");

        // Decode using ABI (EXACTLY like Fibonacci)
        let public_values = SolLockPublicValues::abi_decode(&output, true).unwrap();
        println!("   Valid: {}", public_values.isValid);
        println!(
            "   Merkle Root: {}",
            hex::encode(public_values.merkleRoot.0)
        );
        println!("   Amount: {}", public_values.amount);
        println!("   Receiver: {}", hex::encode(public_values.receiver.0));
        println!("   Slot: {}", public_values.slot);
        println!("📊 Cycles: {}", report.total_instruction_count());
    } else {
        // Generate proof
        let (pk, vk) = client.setup(SOL_LOCK_ELF);
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("✅ Proof generated successfully!");

        // Verify proof
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("✅ Proof verified successfully!");

        // Decode public values using ABI
        let public_values = SolLockPublicValues::abi_decode(&proof.public_values, true).unwrap();
        println!("   Valid: {}", public_values.isValid);
        println!(
            "   Merkle Root: {}",
            hex::encode(public_values.merkleRoot.0)
        );
        println!("   Amount: {}", public_values.amount);
        println!("   Receiver: {}", hex::encode(public_values.receiver.0));
        println!("   Slot: {}", public_values.slot);
    }
}
