//! A zk proof that verifies a SOL lock transaction with Merkle proof

#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use sp1_zkvm::io;

// Solidity-compatible struct for Ethereum verification
sol! {
    struct SolLockProof {
        bool isValid;
        bytes32 merkleRoot;
        uint64 amount;
        bytes20 receiver;
        uint64 slot;
    }
}

pub fn main() {
    // Read the input data (Merkle proof + transaction data)
    let merkle_root = io::read::<[u8; 32]>();
    let merkle_proof_len = io::read::<u32>();
    let mut merkle_proof = Vec::new();
    for _ in 0..merkle_proof_len {
        merkle_proof.push(io::read::<[u8; 32]>());
    }
    let leaf_data = io::read::<[u8; 32]>();
    let leaf_index = io::read::<u32>();

    let sender = io::read::<[u8; 32]>();
    let receiver = io::read::<[u8; 20]>();
    let amount = io::read::<u64>();
    let slot = io::read::<u64>();

    // 1. Verify Merkle proof (simplified for dummy data)
    let merkle_valid =
       verify_merkle2. Verify bu amount_val);
let receiver_valid = receiver != [0u8; 20];

    let is_valid = merkle_valid && amount_valid && sender_valid && receiver_valid;

    // Create the proof output
let proof = SolLockProof {
        isValid: is_valid,
        merkleRoot: merkle_root.into(),
        amount,
        receiver: receiver.into(),
        slot,
    };

    // Encode and commit the public values (just like Fibonacci example)
    let bytes = SolLockProof::abi_encode(&proof);
    io::commit_slice(&bytes);
}

// Dummy Merkle verification for testing
fn verify_merkle_proof_dummy(
    root: &[u8; 32],
    proof: &Vec<[u8; 32]>,
    leaf: &[u8; 32],
    index: u32,
) -> bool {
    // For dummy data, just check basic consistency
    // In production, this would do full Merkle path verification
    !proof.is_empty() && root != &[0u8; 32] && leaf != &[0u8; 32]
}

