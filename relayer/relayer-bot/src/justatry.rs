use merkle_light::merkle::MerkleTree;
use merkle_light::hash::Algorithm;
use sha2::{Sha256, Digest};
use hex::encode;
use std::hash::Hasher;
use anyhow::{Result, anyhow};
use solana_sdk::signature::Signature;
use std::str::FromStr;

// Your custom SHA256 algorithm (keep this, it's good!)
#[derive(Clone)]
pub struct Sha256Algorithm {
    hasher: Sha256,
}

impl Default for Sha256Algorithm {
    fn default() -> Self {
        Sha256Algorithm {
            hasher: Sha256::new(),
        }
    }
}

impl Hasher for Sha256Algorithm {
    fn finish(&self) -> u64 {
        let result = self.hasher.clone().finalize();
        u64::from_le_bytes(result[0..8].try_into().unwrap())
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}

impl Algorithm<[u8; 32]> for Sha256Algorithm {
    fn hash(&mut self) -> [u8; 32] {
        let result = self.hasher.clone().finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    fn reset(&mut self) {
        self.hasher = Sha256::new();
    }

    fn leaf(&mut self, leaf: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(leaf);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }

    fn node(&mut self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

// Transaction data structure for clarity
#[derive(Debug, Clone)]
pub struct TransactionLeaf {
    pub signature: String,
    pub slot: u64,
    pub leaf_hash: [u8; 32],
}

impl TransactionLeaf {
    // Create leaf from transaction data
    pub fn new(signature: String, slot: u64) -> Result<Self> {
        // Decode signature from base58
        let sig_bytes = bs58::decode(&signature)
            .into_vec()
            .map_err(|e| anyhow!("Invalid signature format: {}", e))?;
        
        // Create leaf data: signature_bytes + slot_bytes
        let mut leaf_data = Vec::new();
        leaf_data.extend_from_slice(&sig_bytes);
        leaf_data.extend_from_slice(&slot.to_le_bytes());
        
        // Hash the complete leaf data
        let mut hasher = Sha256::new();
        hasher.update(&leaf_data);
        let result = hasher.finalize();
        
        let mut leaf_hash = [0u8; 32];
        leaf_hash.copy_from_slice(&result);
        
        Ok(Self {
            signature,
            slot,
            leaf_hash,
        })
    }
    
    // Get the hash for tree building
    pub fn hash(&self) -> [u8; 32] {
        self.leaf_hash
    }
}

// Merkle proof structure with metadata
#[derive(Debug, Clone)]
pub struct MerkleProofData {
    pub transaction: TransactionLeaf,
    pub tx_index: usize,
    pub proof: Vec<[u8; 32]>,
    pub root: [u8; 32],
    pub total_transactions: usize,
}

// ✅ FIXED: Build Merkle tree from transactions
pub async fn create_merkle_tree_from_txs(
    rpc_url: &str,
    tx_signatures: Vec<&str>,
) -> Result<(MerkleTree<[u8; 32], Sha256Algorithm>, Vec<TransactionLeaf>)> {
    
    if tx_signatures.is_empty() {
        return Err(anyhow!("No transactions provided"));
    }
    
    println!("🔨 Building Merkle tree from {} transactions...", tx_signatures.len());
    
    let mut transaction_leaves = Vec::new();
    
    // Fetch and hash each transaction
    for (index, sig) in tx_signatures.iter().enumerate() {
        match fetch_tx_and_block_header(rpc_url, sig).await {
            Ok((signature, slot)) => {
                // Create leaf with proper hashing
                let leaf = TransactionLeaf::new(signature, slot)?;
                
                println!("  ✅ Tx {}: {} (slot: {}, hash: {})", 
                    index, 
                    &leaf.signature[..8], 
                    leaf.slot,
                    encode(&leaf.leaf_hash[..8])
                );
                
                transaction_leaves.push(leaf);
            }
            Err(e) => {
                println!("  ❌ Failed to fetch tx {}: {}", sig, e);
                return Err(anyhow!("Failed to fetch transaction {}: {}", sig, e));
            }
        }
    }
    
    // Build Merkle tree from leaf hashes
    let leaf_hashes: Vec<[u8; 32]> = transaction_leaves
        .iter()
        .map(|leaf| leaf.hash())
        .collect();
    
    let tree: MerkleTree<[u8; 32], Sha256Algorithm> = 
        MerkleTree::from_iter(leaf_hashes.into_iter());
    
    let root = tree.root();
    println!("\n🌳 Merkle Root: {}", encode(root));
    println!("   Tree height: {}", (transaction_leaves.len() as f64).log2().ceil() as usize);
    
    Ok((tree, transaction_leaves))
}

// ✅ ENHANCED: Generate proof with full metadata
pub fn generate_proof_for_tx(
    tree: &MerkleTree<[u8; 32], Sha256Algorithm>,
    transaction_leaves: &[TransactionLeaf],
    tx_index: usize,
) -> Result<MerkleProofData> {
    
    if tx_index >= transaction_leaves.len() {
        return Err(anyhow!(
            "Transaction index {} out of bounds (total: {})", 
            tx_index, 
            transaction_leaves.len()
        ));
    }
    
    println!("\n🔍 Generating proof for transaction at index {}...", tx_index);
    
    // Generate proof
    let proof = tree.gen_proof(tx_index);
    let proof_hashes = proof.lemma().to_vec();
    
    println!("   Transaction: {}", transaction_leaves[tx_index].signature);
    println!("   Slot: {}", transaction_leaves[tx_index].slot);
    println!("   Proof length: {} hashes", proof_hashes.len());
    println!("   Sibling hashes:");
    
    for (i, hash) in proof_hashes.iter().enumerate() {
        println!("     [{}] {}", i, encode(hash));
    }
    
    let proof_data = MerkleProofData {
        transaction: transaction_leaves[tx_index].clone(),
        tx_index,
        proof: proof_hashes,
        root: *tree.root(),
        total_transactions: transaction_leaves.len(),
    };
    
    Ok(proof_data)
}

// ✅ NEW: Verify proof locally (before sending to EVM/ZK)
pub fn verify_merkle_proof(proof_data: &MerkleProofData) -> Result<bool> {
    println!("\n✓ Verifying Merkle proof...");
    
    let mut current_hash = proof_data.transaction.leaf_hash;
    println!("   Starting with leaf: {}", encode(&current_hash[..8]));
    
    // Climb up the tree using proof
    for (i, sibling) in proof_data.proof.iter().enumerate() {
        let mut hasher = Sha256::new();
        
        // Determine order (left/right) based on index
        let index_at_level = proof_data.tx_index >> i;
        
        if index_at_level % 2 == 0 {
            // Current is left, sibling is right
            hasher.update(&current_hash);
            hasher.update(sibling);
            println!("   Level {}: hash(current + sibling)", i);
        } else {
            // Sibling is left, current is right
            hasher.update(sibling);
            hasher.update(&current_hash);
            println!("   Level {}: hash(sibling + current)", i);
        }
        
        let result = hasher.finalize();
        current_hash.copy_from_slice(&result);
        println!("     → {}", encode(&current_hash[..8]));
    }
    
    let matches = current_hash == proof_data.root;
    
    if matches {
        println!("   ✅ Proof is VALID! Computed root matches.");
    } else {
        println!("   ❌ Proof is INVALID!");
        println!("     Expected: {}", encode(&proof_data.root));
        println!("     Got:      {}", encode(&current_hash));
    }
    
    Ok(matches)
}

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