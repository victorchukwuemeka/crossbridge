use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

pub async fn fetch_tx_and_block_header(rpc_url:&str,tx_signature: &str){
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
    let block = client.get_block(slot).await.expect("Failed to fetch block");

    println!("Block header (slot {}): {:?}", slot, block.blockhash);



}