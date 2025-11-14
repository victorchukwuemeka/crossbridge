use merkle_light::merkle::MerkleTree;
use merkle_light::hash::Algorithm;
use sha2::{Sha256, Digest};
use hex::encode;
use std::hash::Hasher;
use anyhow::{Result, anyhow};
use solana_sdk::signature::Signature;
use std::str::FromStr;

use crate::fetch_tx_and_block_header::fetch_tx_and_block_header;


#[derive(Clone)]
pub struct Sha256Algorithm{
    hasher :Sha256,
}

impl Default for Sha256Algorithm {
    fn default() -> Self {
        Sha256Algorithm{
            hasher :Sha256::new(),
        }
    }
}

//from the Algorithms documentation to implemet it hasher must be implemented first 
impl Hasher for Sha256Algorithm{
    fn finish(&self) -> u64 {
        let result = self.hasher.clone().finalize();
        u64::from_le_bytes(result[0..8].try_into().unwrap())
    }
    
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}


/** the Algorithm that we implemented is a trait ffor hashing streams of bytes  */
impl Algorithm<[u8; 32]> for Sha256Algorithm where Self: Hasher{
    fn hash(&mut self) -> [u8; 32] {
        //let result = self.0.clone().finalize();
        let result = self.hasher.clone().finalize(); 
        let mut hash =[0u8; 32];
        hash.copy_from_slice(&result);
        hash

    }

    fn reset(&mut self) {
        self.hasher = Sha256::new();
    }
    
    /*
     * the leaf node is a hash of the leaf 
     * 
     */
    fn leaf(&mut self, leaf: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(leaf);
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash 
    }
    
    /** the interior  node or parent node that is what i' talking about here */
    fn node(&mut self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        let result  = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}


//the data structure of the transactoin so everything will be clear 
#[derive(Debug, Clone)]
pub struct TransactionLeaf{
    pub signature : String,
    pub slot : u64,
    pub leaf_hash : [u8; 32],
}

/*
 * the transactionleaf helps us convert the transaction 
 * into a hash format that we can now
 * pass to our merkle_algorithm leaf 
 * 
 * 1. decode signature into vec 
 * 2. create leaf data = signature_bytes + slot_bytes
 * 3. hash the complete leaf data 
 * 4. 
*/
impl TransactionLeaf {
    pub fn new(signature:String, slot:u64)->Result<Self>{
        //the signature  is still in base58 and we have  decode to vec
        let sig_bytes = bs58::decode(&signature)
         .into_vec()
         .map_err(|e| anyhow!("invalid signature format{}", e))?;

        //our lead data 
        let mut leaf_data = Vec::new();
        leaf_data.extend_from_slice(&sig_bytes);
        leaf_data.extend_from_slice(&slot.to_le_bytes());

        //hash the complete leaf data 
        let mut hasher = Sha256::new();
        hasher.update(&leaf_data);
        let result = hasher.finalize();

        let mut leaf_hash = [0u8; 32];
        leaf_hash.copy_from_slice(&result);


        Ok(Self { signature, slot, leaf_hash })

    }
   
    //get the hash for building the tree
    pub fn hash(&self)->[u8; 32]{
        self.leaf_hash
    }
    
}


//structure of the merkle proof 
#[derive(Debug, Clone)]
pub struct MerkleProofData{
    pub transaction: TransactionLeaf,
    pub tx_index: usize,
    pub proof : Vec<[u8; 32]>,
    pub root: [u8; 32],
    pub total_transactions: usize,
}


pub async fn create_merkle_tree_from_txs(
    rpc_url:&str, 
    tx_signatures: Vec<&str>
)->Result<(MerkleTree<[u8; 32], Sha256Algorithm>, Vec<TransactionLeaf>)>{

    if tx_signatures.is_empty() {
        return Err(anyhow!("No transactions provided"));
    }

    println!("Building Merkle from transaction{}", tx_signatures.len());
    
    let mut transaction_leaves = Vec::new();

    for (index, sig) in tx_signatures.iter().enumerate() {
    match fetch_tx_and_block_header(rpc_url, sig).await {
        Ok((signature, slot)) => {
            // Create the leaf and handle potential errors
            match TransactionLeaf::new(signature, slot) {
                Ok(leaf) => {
                    println!(
                        "  ✅ Tx {}: {} (slot: {}, hash: {})",
                        index,
                        &leaf.signature[..8],
                        leaf.slot,
                        encode(&leaf.leaf_hash[..8])
                    );
                    transaction_leaves.push(leaf);
                }
                Err(e) => {
                    println!(" Failed to create leaf for tx {}: {}", sig, e);
                    return Err(anyhow!("Failed to create leaf for tx {}: {}", sig, e));
                }
            }
        }
        Err(e) => {
            println!(" Failed to fetch tx {}: {}", sig, e);
            return Err(anyhow!("Failed to fetch transaction {}: {}", sig, e));
        }
    }
}


    //we are to build the merkle from the leave hashes
    let leaf_hashes:Vec<[u8; 32]> = transaction_leaves
     .iter()
     .filter_map(|res| Some(res))
     .map(|leave|leave.hash())
     .collect();

     
    let tree: MerkleTree<[u8; 32], Sha256Algorithm> = 
        MerkleTree::from_iter(leaf_hashes.into_iter());
    
    let root = tree.root();

    println!("\n🌳 Merkle Root: {}", encode(root));
    println!("   Tree height: {}", (transaction_leaves.len() as f64).log2().ceil() as usize);
    
    Ok((tree, transaction_leaves))
    
    
}







pub fn generate_proof_for_tx(
    tree: &MerkleTree<[u8; 32], Sha256Algorithm>,
    transaction_leaves: &[TransactionLeaf],
    tx_index : usize
)->Result<MerkleProofData>{

    if tx_index >= transaction_leaves.len(){
        return Err(anyhow!(
            "Transaction index {} out of bounds (total: {})", 
            tx_index, 
            transaction_leaves.len()
        ));
    }

    println!("\n🔍 Generating proof for transaction at index {}...", tx_index);
    

    let proof = tree.gen_proof(tx_index);
    let proof_hashes = proof.lemma().to_vec();

    println!("   Transaction: {}", transaction_leaves[tx_index].signature);
    println!("   Slot: {}", transaction_leaves[tx_index].slot);
    println!("   Proof length: {} hashes", proof_hashes.len());
    println!("   Sibling hashes:");

    for (i, hash) in proof_hashes.iter().enumerate() {
        println!("[{}], {}", i, encode(hash));
    }

    let proof_data = MerkleProofData{
        transaction : transaction_leaves[tx_index].clone(),
        tx_index,
        proof: proof_hashes,
        root: tree.root(),
        total_transactions: transaction_leaves.len(),
    };

    Ok(proof_data)
}