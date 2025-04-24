use web3::types::{H160, U256};
use std::error::Error;
use std::rc::Rc;
use std::str::FromStr;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::Keypair;

pub async fn unlock(user: H160, amount: U256, solana_address: String) -> Result<(), Box<dyn Error>> {
    println!("🔓 Unlocking {} SOL for {} (Solana address: {})", amount, user, solana_address);

    

    let user_pubkey = Pubkey::from_str(&user)?;
    let solana_address = Pubkey::from_str(&solana_address)?;


    
    // Load key for Solana config part
    let key_pair = "/home/victor/.config/solana/id.json";
    let file = File::open(key_pair)?;
    let keypair_bytes: Vec<u8> = serde_json::from_reader(file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

     // Network local  net  for now
     let client = Client::new_with_options(
        Cluster::Custom(
            "http://127.0.0.1:8899".to_string(), // RPC URL
            "ws://127.0.0.1:8900".to_string()    // WebSocket URL
        ),
        Rc::new(keypair),
        CommitmentConfig::processed(),
    );

    let my_instruction = system_instruction::transfer{
        &solana_address,
        &user_pubkey
        amount
    }

    // Wrap it in a transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );


    // Send the transaction
    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("✅ Unlock successful! Tx Signature: {}", signature);
    
    Ok(())
}
