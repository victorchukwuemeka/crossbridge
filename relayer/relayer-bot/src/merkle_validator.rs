use anyhow::Ok;
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
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
    



/**
 * just making sure the block transaction is actually in solana 
 * 
 */
pub async fn validate_transaction_in_block(
    rpc_url: &str,
    signature: &str,
    slot: u64,
)->Result<bool>{

    let rpc_client = RpcClient::new(rpc_url.to_string());

    // Get block config 
    let config = solana_client::rpc_config::RpcBlockConfig {
        encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
        transaction_details: Some(solana_transaction_status::TransactionDetails::Signatures),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

   let block = rpc_client.get_block_with_config(slot, config)?;

   //getting the signatures from the block 
   let all_signatures: Vec<String> = block.signatures
   .unwrap_or_default()
   .iter()
   .map(|sig|sig.to_string()).collect();
  
    let tx_index = all_signatures.iter()
    .position(|sig| sig==signature)
    .ok_or_else(||anyhow!("transaction not found in bock"))?;


   //build the merkle tree
   let (tree, leaves) = create_merkle_tree_from_txs(
        rpc_url,
        all_signatures.iter().map(|s| s.as_str()).collect(),
    ).await?;
    

    //generate prooofs from the merkle tree created 
    let proof_data  = generate_proof_for_tx(&tree, &leaves, tx_index)?;


    //verify merkle proof ith proof data 
    let is_valid = verify_merkle_proof(&proof_data)?;
    assert!(is_valid, "proof as valid !");


    
    Ok(true)
}


//#[tokio::main]
/*
*
async fn main() -> Result<()> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let rpc_client = RpcClient::new(rpc_url.to_string());

    
    // 1. Get block and extract transaction signatures
   
    // Get block config
    let config = solana_client::rpc_config::RpcBlockConfig {
        encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
        transaction_details: Some(solana_transaction_status::TransactionDetails::Signatures),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    //
    let slot = rpc_client.get_slot()?;
    println!("Latest slot: {}", slot);
    
    
    
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
*/