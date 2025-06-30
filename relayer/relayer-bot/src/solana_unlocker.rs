use solana_sdk::signer::Signer;
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
use solana_sdk::native_token::LAMPORTS_PER_SOL;

pub async fn unlock(user: String, amount: u64, solana_address: String) -> Result<(), Box<dyn Error>> {
    println!("🔓 Unlocking {} SOL for {} (Solana address: {})", amount, user, solana_address);

    let amount_in_lamport = amount * LAMPORTS_PER_SOL;
    
   // let user_hash = 
     println!("The User :{}", user);
    //let user_pubkey = Pubkey::from_str(&user)?;
    let solana_address_pubkey = match Pubkey::from_str(&solana_address) {
        Ok(user_pubkey)=>{
            println!("User Public Key :{}", user_pubkey);
            user_pubkey
        }
        Err(e)=>{
            println!("we could not get the pubkey :{}", e);
            return Err(e.into()); 
        }
    };
    //println!("User Public Key : {}", user_pubkey);
    let solana_address_pubkey = Pubkey::from_str(&solana_address)?;
    println!("the solana Address Pubkey :{}", solana_address_pubkey);

    
    // Load key for Solana config part
    let key_pair = "/home/victor/.config/solana/id.json";
    let file = File::open(key_pair)?;
    let keypair_bytes: Vec<u8> = serde_json::from_reader(file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;
    println!("Keypair :{:?}", keypair);

     // Network local  net  for now
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::processed(),
    );
    //println!("rpc_client {:?}", rpc_client);
     /*let client = Client::new_with_options(
        Cluster::Custom(
            "http://127.0.0.1:8899".to_string(), // RPC URL
            "ws://127.0.0.1:8900".to_string()    // WebSocket URL
        ),
        Rc::new(keypair),
        CommitmentConfig::processed(),
    );*/
    
    let test_amount = 10_000_000_u64; 

    let my_instruction = system_instruction::transfer(
         &keypair.pubkey(),   
        &solana_address_pubkey,
        test_amount,
    );

    //print!("this is the instruction:{}:", my_instruction);
    println!("This is the Instruction {:?}:", my_instruction);
    // Wrap it in a transaction
    // ✅ Get the latest blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    println!("The Recent Block Hash {}:", recent_blockhash);

    let tx = Transaction::new_signed_with_payer(
        &[my_instruction],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );
    println!("TRANSACTION {:?}", tx);


    // Send the transaction
    let signature = rpc_client.send_and_confirm_transaction(&tx)?;
    println!("✅ Unlock successful! Tx Signature: {}", signature);
    
    Ok(())
}
