use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;
use std::error::Error;
use solana_client::rpc_config::RpcBlockConfig;
use solana_transaction_status::{TransactionDetails};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_client::rpc_config::RpcTransactionConfig;



/** 
pub async fn fetch_tx_and_block_header(rpc_url:&str,tx_signature: &str)->Result<(String, u64), Box<dyn Error>>{
   //let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let signature = Signature::from_str(tx_signature).unwrap();
    
    // 1. Fetch the transaction
    //return EncodedConfirmedTransactionWithStatusMeta which is a struct that has a slot
    //slot is in u64
    let tx = client
        .get_transaction(&signature, UiTransactionEncoding::Json)
        .await
        .expect("Failed to fetch transaction");

    println!("Transaction: {:?}", tx);

    let slot  = tx.slot;
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::finalized()),
        max_supported_transaction_version: Some(0),  // ← ADD THIS
    };
    let block = client.get_block_with_config(slot, config)?;

    //let block = client.get_block(slot).await.expect("Failed to fetch block");


    println!("Block header (slot {}): {:?}", slot, block.blockhash);
    Ok((tx_signature.to_string(), slot))

}
*/


pub async fn fetch_tx_and_block_header(rpc_url: &str, tx_signature: &str) -> Result<(String, u64), Box<dyn Error>> {
    let client = RpcClient::new(rpc_url.to_string());
    let signature = Signature::from_str(tx_signature)?;  
    
    // Fetch the transaction
     let tx_config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::finalized()),
        max_supported_transaction_version: Some(0),  // ← ADD THIS
    };
    let tx = client
        .get_transaction_with_config(&signature, tx_config)
        .await?;

    /*let tx = client
        .get_transaction(&signature, UiTransactionEncoding::Json)
        .await
        .expect("Failed to fetch transaction");*/
    
    println!("Transaction: {:?}", tx);
    let slot = tx.slot;
    
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Json),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(false),
        commitment: Some(CommitmentConfig::finalized()),
        max_supported_transaction_version: Some(0),
    };
    
    let block = client.get_block_with_config(slot, config).await?;  // Add .await here
    
    println!("Block header (slot {}): {:?}", slot, block.blockhash);
    Ok((tx_signature.to_string(), slot))
}