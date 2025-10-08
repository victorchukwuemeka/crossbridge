//! Generate a zk proof for SOL lock verification with Merkle proofs

use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF file for our SOL lock program
pub const SOL_LOCK_ELF: &[u8] = include_elf!("sol-lock-program");

/// Solidity-compatible output structure
#[derive(Debug)]
struct SolLockProof {
    is_valid: bool,
    merkle_root: [u8; 32],
    amount: u64,
    receiver: [u8; 20],
    slot: u64,
}

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
    // Setup logger
    sp1_sdk::utils::setup_logger();

    // Parse arguments
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: Specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup prover client
    let client = ProverClient::from_env();

    // Create dummy Merkle proof data
    let dummy_merkle_root = [1u8; 32];
    let dummy_merkle_proof = vec![[2u8; 32], [3u8; 32]];
    let dummy_leaf_data = [4u8; 32];
    let dummy_leaf_index = 0u32;
    let dummy_tree_size = 4u32;

    // Create dummy transaction data
    let dummy_sender = [5u8; 32];
    let dummy_receiver = [6u8; 20];

    // Setup inputs
    let mut stdin = SP1Stdin::new();
    stdin.write(&dummy_merkle_root);
    stdin.write(&(dummy_merkle_proof.len() as u32));
    for item in &dummy_merkle_proof {
        stdin.write(item);
    }
    stdin.write(&dummy_leaf_data);
    stdin.write(&dummy_leaf_index);
    stdin.write(&dummy_tree_size);
    stdin.write(&dummy_sender);
    stdin.write(&dummy_receiver);
    stdin.write(&args.amount);
    stdin.write(&args.slot);

    println!("🔐 Generating SOL lock proof with:");
    println!("   Amount: {} lamports", args.amount);
    println!("   Slot: {}", args.slot);
    println!("   Receiver: {}", hex::encode(dummy_receiver));

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(SOL_LOCK_ELF, &stdin).run().unwrap();
        println!("✅ Program executed successfully");
        println!("📊 Cycles: {}", report.total_instruction_count());

        // Decode output
        let proof = decode_sol_lock_output(&output);
        println!("   Valid: {}", proof.is_valid);
        println!("   Merkle Root: {}", hex::encode(proof.merkle_root));
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

        // Decode and show results
        let result = decode_sol_lock_output(&proof.public_values);
        println!("   Valid: {}", result.is_valid);
        println!("   Merkle Root: {}", hex::encode(result.merkle_root));
        println!("   Amount: {}", result.amount);
        println!("   Receiver: {}", hex::encode(result.receiver));
        println!("   Slot: {}", result.slot);
    }
}

/// Decode the SOL lock proof output
fn decode_sol_lock_output(output: &[u8]) -> SolLockProof {
    // For now, use simple encoding. Later we can use ABI encoding like Fibonacci
    let is_valid = output[0] != 0;
    let mut merkle_root = [0u8; 32];
    merkle_root.copy_from_slice(&output[1..33]);
    let amount = u64::from_le_bytes(output[33..41].try_into().unwrap());
    let mut receiver = [0u8; 20];
    receiver.copy_from_slice(&output[41..61]);
    let slot = u64::from_le_bytes(output[61..69].try_into().unwrap());

    SolLockProof {
        is_valid,
        merkle_root,
        amount,
        receiver,
        slot,
    }
}
