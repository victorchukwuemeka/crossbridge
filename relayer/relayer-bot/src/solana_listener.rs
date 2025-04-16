use std::error::Error;
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use solana_sdk::commitment_config::CommitmentConfig;
use std::fs::File;
use std::rc::Rc;
use serde_json;

pub async fn start()->Result<(), Box<dyn Error>>{
    println!("STARTED Listening ");
    //load key for the solana config part
    let key_pair = "/home/victor/.config/solana/id.json";
    let file = File::open(key_pair)?;
    let keypair_bytes: Vec<u8> = serde_json::from_reader(file);
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

    
    

    println!("✅ Ethereum BurnEvent handled!");
    Ok(())
}