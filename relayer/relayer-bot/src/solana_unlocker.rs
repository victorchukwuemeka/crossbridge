use web3::types::{H160, U256};
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::Keypair;
use std::fs::File;
use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use anchor_client::anchor_lang::Key;

pub async fn unlock(user: String, amount: u64, solana_address: String) -> Result<(), Box<dyn Error>> {
    println!("🔓 Unlocking {} SOL for {} (Solana address: {})", amount, user, solana_address);

    
   // let user_hash = 
    let user_pubkey = Pubkey::from_str(&user)?;
    let solana_address_pubkey = Pubkey::from_str(&solana_address)?;

    
    // Load key for Solana config part
    let key_pair = "/home/victor/.config/solana/id.json";
    let file = File::open(key_pair)?;
    let keypair_bytes: Vec<u8> = serde_json::from_reader(file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

     // Network local  net  for now
      let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::processed(),
    );
     /*let client = Client::new_with_options(
        Cluster::Custom(
            "http://127.0.0.1:8899".to_string(), // RPC URL
            "ws://127.0.0.1:8900".to_string()    // WebSocket URL
        ),
        Rc::new(keypair),
        CommitmentConfig::processed(),
    );*/

    let my_instruction = system_instruction::transfer(
        &solana_address_pubkey,
        &user_pubkey,
        amount,
    );

    // Wrap it in a transaction
    // ✅ Get the latest blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[my_instruction],
        Some(&solana_address_pubkey.key()),
        &[&keypair],
        recent_blockhash,
    );


    // Send the transaction
    let signature = rpc_client.send_and_confirm_transaction(&tx)?;
    println!("✅ Unlock successful! Tx Signature: {}", signature);
    
    Ok(())
}
