//use crate::merkle::MerkleProofData;
use hex::encode;

use anyhow::Result;
use anyhow::anyhow;
//use crate::fetch_tx_and_block_header::fetch_tx_and_block_header;
//use crate::merkle::create_merkle_tree_from_txs;
//use crate::merkle::generate_proof_for_tx;
//use crate::verify_merkle_proof::verify_merkle_proof;
use relayer_bot::merkle::MerkleProofData;
use relayer_bot::merkle::create_merkle_tree_from_txs;
use relayer_bot::merkle::generate_proof_for_tx;
use relayer_bot::verify_merkle_proof::verify_merkle_proof;
//use relayer_bot::fetch_tx_and_block_header::fetch_tx_and_block_header;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
    

/**
 * serialization of proof structure   
 */
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


/**async fn fetch_tx_and_block_header(
    rpc_url: &str,
    signature: &str,
) -> Result<(String, u64)> {
    // Your existing implementation, but return Result
    // This should fetch from RPC and return (signature, slot)
    crate::fetch_tx_and_block_header::fetch_tx_and_block_header(rpc_url, signature)
        .await
        .map_err(|e| anyhow!("RPC fetch failed: {}", e))
}*/



#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let rpc_client = RpcClient::new(rpc_url.to_string());

    let slot = rpc_client.get_slot()?;
    println!("Latest slot: {}", slot);
    
    // 1. Get block and extract transaction signatures
    /*let tx_signatures = vec![
        "5J8H5sTvEhnGcB7vFiqz7FPRmEfS7Ctr1h3qH7dKPz8LmXc...",
        "3kFq9xQvP7FxH8nMqRz4TpYbKjE2cWdLhXvN9sG1mPtR...",
        "2mNzW8pKxRqYvH6TcBjF9sL4nXeG7vQhMtPkD5wJ3rS...",
        // ... more signatures
    ];*/

    /*let real_signature = "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW";
    
    let sig2 = "2".repeat(88);
    let sig3 = "3".repeat(88);
    let sig4 = "4".repeat(88);
    
    let tx_signatures: Vec<&str> = vec![
        real_signature,
        &sig2,
        &sig3,
        &sig4,
    ];*/    
    /*let tx_signatures = vec![
        real_signature.to_string(),
        &"2".repeat(88), // Mock signature (valid length)
        &"3".repeat(88), // Mock signature
        &"4".repeat(88), // Mock signature
    ];*/
    /*let tx_signatures: Vec<&str> = vec![
        "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW",
        "4hXTCkRzt9WyecNzV1XPgCDfGAZzQKNxLXgynz5QaaRAxCDZhaGT8DapqVXXXJvpXrZwpPZGRDGhv4K6RqEXZLgp",
        "3nxbXGxVJm9RYkcKCWQKJJnKcXKvJ8JnKcXKvJ8JnKcXKvJ8JnKcXKvJ8JnKcXKvJ8JnKcXKvJ8JnKcXKvJ8Jn",
        "2ZzZqYZ7fXmQpJxVZ9qYZ7fXmQpJxVZ9qYZ7fXmQpJxVZ9qYZ7fXmQpJxVZ9qYZ7fXmQpJxVZ9qYZ7fXmQpJxVZ",
    ];*/

    //let block = rpc_client.get_block(slot)?;

    // Get block with confirmed transactions
    let config = solana_client::rpc_config::RpcBlockConfig {
        encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
        transaction_details: Some(solana_transaction_status::TransactionDetails::Signatures),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };
    
    
    // Extract signatures
    let block = rpc_client.get_block_with_config(slot, config)?;
    
    // Extract just the signatures
    let tx_signatures: Vec<String> = block.signatures
        .unwrap_or_default()
        .iter()
        .take(4)
        .map(|sig| sig.to_string())
        .collect();

    // 2. Build Merkle tree
    let (tree, leaves) = create_merkle_tree_from_txs(
        rpc_url,
        tx_signatures.iter().map(|s| s.as_str()).collect(),
    ).await?;
    
    // 3. Generate proof for your bridge transaction (e.g., index 0)
    let proof_data = generate_proof_for_tx(&tree, &leaves, 0)?;
    
    // 4. Verify proof locally
    let is_valid = verify_merkle_proof(&proof_data)?;
    assert!(is_valid, "Proof must be valid!");
    
    // 5. Serialize for ZK circuit or EVM
    //let serialized: SerializedProof = proof_data.into();
    //serialized.save_to_file("proof.json")?;
    
    println!("\n🎉 Merkle proof ready for ZK circuit!");
    
    Ok(())
}

