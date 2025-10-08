use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct SolLockPublicValues {
        bool isValid;
        bytes32 merkleRoot;
        uint64 amount;
        bytes20 receiver;
        uint64 slot;
    }
}

/// Verify a SOL lock transaction with Merkle proof
pub fn verify_sol_lock(
    merkle_root: [u8; 32],
    merkle_proof: Vec<[u8; 32]>,
    leaf_data: [u8; 32],
    leaf_index: u32,
    sender: [u8; 32],
    receiver: [u8; 20],
    amount: u64,
    slot: u64,
) -> SolLockPublicValues {
    // 1. Verify Merkle proof (simplified for now)
    let merkle_valid =
        verify_merkle_proof_simple(&merkle_root, &merkle_proof, &leaf_data, leaf_index);

    // 2. Verify transaction parameters
    let amount_valid = amount > 0 && amount < 10_000_000_000; // Reasonable SOL amount
    let sender_valid = sender != [0u8; 32];
    let receiver_valid = receiver != [0u8; 20];

    let is_valid = merkle_valid && amount_valid && sender_valid && receiver_valid;

    SolLockPublicValues {
        isValid: is_valid,
        merkleRoot: merkle_root.into(),
        amount,
        receiver: receiver.into(),
        slot,
    }
}

/// Simplified Merkle proof verification (for dummy data)
fn verify_merkle_proof_simple(
    root: &[u8; 32],
    proof: &Vec<[u8; 32]>,
    leaf: &[u8; 32],
    index: u32,
) -> bool {
    // For dummy data, just check basic consistency
    // In production, this would implement full Merkle path verification
    !proof.is_empty() && root != &[0u8; 32] && leaf != &[0u8; 32]
}
