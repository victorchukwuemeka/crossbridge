use anyhow::Result;
//use rs_merkle::algorithms::Sha256;
//use sha2::{Digest};
use sha2::{Sha256, Digest};

use hex::encode;

use crate::merkle::TransactionLeaf;

#[derive(Debug, Clone)]
pub struct MerkleProofData{
    pub transactions : TransactionLeaf,
    pub tx_index: usize,
    pub proof: Vec<[u8; 32]>,
    pub root: [u8; 32],
    pub total_transactions: usize,
}


pub fn verify_merkle_proof(proof_data : &MerkleProofData)->Result<bool>{
    println!("\n✓ Verifying Merkle proof...");

    let mut current_hash = proof_data.transactions.leaf_hash;

    for (i, sibling) in proof_data.proof.iter().enumerate() {
        let mut hasher = Sha256::new();
        let index_at_level = proof_data.tx_index >> i;

        if index_at_level % 2 == 0 {
            hasher.update(&current_hash);
            hasher.update(sibling);
        }else {
            hasher.update(sibling);
            hasher.update(&current_hash);
        }

        let result = hasher.finalize();
        current_hash.copy_from_slice(&result);
    }

    let matches = current_hash  == proof_data.root;

     if matches {
        println!("   ✅ Proof is VALID!");
    } else {
        println!("   ❌ Proof is INVALID!");
    }

    Ok(matches)
}