use merkle_light::merkle::MerkleTree;
use merkle_light::hash::Algorithm;
use sha2::{Sha256, Digest};
use hex::encode;
use std::hash::Hasher;

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
        let result  = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}


pub async fn create_merkle_tree_from_txs(
    rpc_url:&str, 
    tx_signatures: Vec<&str>
)->MerkleTree<[u8; 32], Sha256Algorithm>{
    let mut tx_data = Vec::new();
    for sig in tx_signatures{
         let(signature, slot) = fetch_tx_and_block_header(rpc_url, sig).await;
         
        let leaf_data = format!("{}:{}",signature, slot);
        tx_data.push(leaf_data.into_bytes());

    }

    let tree:MerkleTree<[u8; 32], Sha256Algorithm> = MerkleTree::from_iter(
        tx_data.iter().map(|x| {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&x[0..32]);
            arr
        })
    );
    println!("Merkle Root: {}", encode(tree.root()));
    
    tree
    
}

pub fn generate_proof_for_tx(
    tree: &MerkleTree<[u8; 32], Sha256Algorithm>,
    tx_index : usize
)-> Vec<[u8; 32]>{
    let proof = tree.gen_proof(tx_index);
    println!("Merkle proof for transaction at index {} ", tx_index);

    for hash in proof.lemma() {
        println!("{}", encode(hash));
    }
    
    proof.lemma().to_vec()
}