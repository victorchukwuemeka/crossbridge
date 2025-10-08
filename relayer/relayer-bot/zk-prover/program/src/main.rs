// program/src/main.rs
#![no_main]
sp1_zkvm::entrypoint!(main);

use sol_lock_lib::{verify_sol_lock, SolLockPublicValues};
use sp1_zkvm::io;

pub fn main() {
    // Read all inputs (following the Fibonacci pattern)
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

    // Use the library function (just like fibonacci() in the original)
    let public_values = verify_sol_lock(
        merkle_root,
        merkle_proof,
        leaf_data,
        leaf_index,
        sender,
        receiver,
        amount,
        slot,
    );

    // Commit the public values using ABI encoding (EXACTLY like Fibonacci)
    let bytes = SolLockPublicValues::abi_encode(&public_values);
    io::commit_slice(&bytes);
}
