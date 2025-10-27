// both the serialization before zk and calling the local test 




// ✅ NEW: Serialize proof for ZK circuit or EVM
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SerializedProof {
    pub signature: String,
    pub slot: u64,
    pub tx_index: usize,
    pub proof: Vec<String>,  // Hex encoded
    pub root: String,        // Hex encoded
    pub leaf_hash: String,   // Hex encoded
}

impl From<MerkleProofData> for SerializedProof {
    fn from(proof_data: MerkleProofData) -> Self {
        Self {
            signature: proof_data.transaction.signature,
            slot: proof_data.transaction.slot,
            tx_index: proof_data.tx_index,
            proof: proof_data.proof.iter().map(|h| encode(h)).collect(),
            root: encode(&proof_data.root),
            leaf_hash: encode(&proof_data.transaction.leaf_hash),
        }
    }
}

impl SerializedProof {
    // Save to file for ZK circuit
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        println!("💾 Proof saved to: {}", path);
        Ok(())
    }
    
    // Load from file
    pub fn load_from_file(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let proof: Self = serde_json::from_str(&json)?;
        Ok(proof)
    }
}

// Helper: Fetch transaction and block (you already have this, but with error handling)
async fn fetch_tx_and_block_header(
    rpc_url: &str,
    signature: &str,
) -> Result<(String, u64)> {
    // Your existing implementation, but return Result
    // This should fetch from RPC and return (signature, slot)
    crate::fetch_tx_and_block_header::fetch_tx_and_block_header(rpc_url, signature)
        .await
        .map_err(|e| anyhow!("RPC fetch failed: {}", e))
}

Usage Example:
rustuse anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    
    // 1. Get block and extract transaction signatures
    let tx_signatures = vec![
        "5J8H5sTvEhnGcB7vFiqz7FPRmEfS7Ctr1h3qH7dKPz8LmXc...",
        "3kFq9xQvP7FxH8nMqRz4TpYbKjE2cWdLhXvN9sG1mPtR...",
        "2mNzW8pKxRqYvH6TcBjF9sL4nXeG7vQhMtPkD5wJ3rS...",
        // ... more signatures
    ];
    
    // 2. Build Merkle tree
    let (tree, leaves) = create_merkle_tree_from_txs(
        rpc_url,
        tx_signatures,
    ).await?;
    
    // 3. Generate proof for your bridge transaction (e.g., index 0)
    let proof_data = generate_proof_for_tx(&tree, &leaves, 0)?;
    
    // 4. Verify proof locally
    let is_valid = verify_merkle_proof(&proof_data)?;
    assert!(is_valid, "Proof must be valid!");
    
    // 5. Serialize for ZK circuit or EVM
    let serialized: SerializedProof = proof_data.into();
    serialized.save_to_file("proof.json")?;
    
    println!("\n🎉 Merkle proof ready for ZK circuit!");
    
    Ok(())
}

