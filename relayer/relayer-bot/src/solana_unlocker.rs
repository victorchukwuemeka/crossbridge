use solana_sdk::signer::Signer;
use std::env;
use std::error::Error;
use std::str::FromStr;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use anchor_client::solana_sdk::signature::Keypair;
use solana_client::rpc_client::{self, RpcClient};
use solana_sdk::transaction::Transaction;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use crate::burn_tracker::BurnTracker;
use solana_sdk::bs58;
use solana_sdk::instruction::Instruction;
use solana_sdk::instruction::AccountMeta;
use sha2::{Sha256, Digest};



pub async fn unlock(
    user: String, 
    amount: f64, 
    solana_address: String, 
    burn_tx_hash: String
    ) -> Result<(), Box<dyn Error>> {
    


    // devnetwork 
    let rpc_client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::processed(),
    );
    println!("[SUCCESS]:Connected to Solana Devnet");



    //check if already processed
    let tracker = BurnTracker::new();
    match tracker.can_process(&burn_tx_hash){
        Ok(can_process)=>{
            if !can_process {
                println!("Transaction already processed, skipping");
                return Ok(());
            }
        }
        Err(e)=>{
            println!("Error while checking the process {}, you continue",e);
        }
    }

    // loading my keypair from my private key in the env
    let private_key_str = match env::var("DEVNET_PRIVATE_KEY"){
        Ok(key)=>{
            println!("[PrivateKey]: private key loaded");
            key
        }
        Err(e)=>{
            println!("[MISSED]: Missing DEVNET_PRIVATE_KEY environment variable: {}", e);
            return Ok(());
        }
    };
    
    let private_key_bytes = match bs58::decode(&private_key_str).into_vec(){
        Ok(key_bytes)=>{
            println!("[PRIVATE KEY BYTES]: the key is loading in bytes");
            key_bytes
        }

        Err(e)=>{
            println!("[key in byts missed]: {} this not convert ", e);
            return Ok(());
        }
    };


    let complete_keypair = match Keypair::from_bytes(&private_key_bytes){
        Ok(complete ) => {
            println!("[COMPLETE kEYPAIR] :  the keypair worked");
            complete
        }

        Err(e)=>{
            println!("Error while checking the process {}, you continue",e);
            return Ok(());
        }
    };

    let my_relayer_pubkey = complete_keypair.pubkey();

    let user_pubkey = match Pubkey::from_str(&solana_address){
        Ok(user)=>{
            println!("[GOTTEN PUBKEY]: {}", user);
            user
        }

        Err(e)=>{
            println!("{} not gotten ", e);
            return Ok(());
        }
    };
    

   //first get the program id for solana program we want to talk to
   let program_id = match Pubkey::from_str("7N9UCyKUqac5JuEjn4inZcBFhi87FXDRy3rP1mNhTrdB"){
    Ok(program)=>{
        println!("[PROGRAM ID] : {}", program);
        program
    }
    Err(e)=>{
        println!("FAILED TO GET pROGRAM ID {}", e);
        return Ok(());       
    }
   };

   //pda of the  bridge and user accounts i used 
   let (bridge_pda, bridge_bump) = Pubkey::find_program_address(
    &[b"bridge_vault_v2"],
     &program_id);
   let (user_balance_pda,user_balance_bump) = Pubkey::find_program_address(
    &[b"user_balance", user_pubkey.as_ref()],
    &program_id
   );


   //discriminator used for selecting the function we are talking to 
   let discriminator = calculate_anchor_discriminator("un_lock_sol");
   

   let mut data = Vec::new();
   data.extend_from_slice(&discriminator);
   //data.extend_from_slice(&discriminator.bytes()[..8]);
   let amount_lamport = (amount * LAMPORTS_PER_SOL as f64) as u64;
   data.extend_from_slice(&amount_lamport.to_le_bytes());
   //data.extend_from_slice(&((amount * 1e9) as u64).to_le_bytes());
   
   //let my_relayer_pubkey = Pubkey::from_str("4dmQAcJe9Ksh4FtpMMfHajP4ssBhrbNrPrGc3v5jFFSA")?;

   let my_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(bridge_pda, false),
            AccountMeta::new(user_balance_pda, false),
            AccountMeta::new_readonly(my_relayer_pubkey, true),
            AccountMeta::new(user_pubkey, false), // No signature needed!
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data,
    };


    // Check if your program exists
    match rpc_client.get_account(&program_id) {
        Ok(account) => {
            println!("✅ Program exists: {} (executable: {})", program_id, account.executable);
            if !account.executable {
                println!("❌ WARNING: Program account exists but is not executable!");
            }
        },
        Err(e) => {
            println!("❌ Program NOT found: {} - Error: {:?}", program_id, e);
        }
    }
    
    println!("Checking if user has locked SOL...");
    
    // Try to fetch the user balance account
    match rpc_client.get_account_data(&user_balance_pda) {
        Ok(data) => {
            println!("✅ User balance PDA exists with {} bytes of data", data.len());
            // You could deserialize this to see the locked amount
        },
        Err(e) => {
            println!("❌ User balance PDA doesn't exist: {:?}", e);
            println!("User {} needs to lock SOL first before unlocking", solana_address);
            return Ok(()); // Exit early
        }
    }
    
    // Add this debugging code before creating your transaction:
    println!("Checking all account existence...");
    
    // Check bridge PDA
    match rpc_client.get_account(&bridge_pda) {
        Ok(account) => println!("✅ Bridge PDA exists: {} (balance: {} lamports)", bridge_pda, account.lamports),
        Err(_) => println!("❌ Bridge PDA NOT found: {}", bridge_pda),
    }
    
    // Check user balance PDA (we know this exists)
    match rpc_client.get_account(&user_balance_pda) {
        Ok(account) => println!("✅ User balance PDA exists: {} (balance: {} lamports)", user_balance_pda, account.lamports),
        Err(_) => println!("❌ User balance PDA NOT found: {}", user_balance_pda),
    }

    // Check user account
    match rpc_client.get_account(&user_pubkey) {
        Ok(account) => println!("✅ User account exists: {} (balance: {} lamports)", user_pubkey, account.lamports),
        Err(_) => println!("❌ User account NOT found: {}", user_pubkey),
    }

    // Check relayer account
    match rpc_client.get_account(&complete_keypair.pubkey()) {
        Ok(account) => println!("✅ Relayer account exists: {} (balance: {} lamports)", complete_keypair.pubkey(), account.lamports),
        Err(_) => println!("❌ Relayer account NOT found: {}", complete_keypair.pubkey()),
    }

    println!("Solana Address : {}", solana_address);
    println!("Pubkey of the Strcut: {}", user_pubkey);
    println!("Exstract the bytes: {:?}", user_pubkey.as_ref());
    println!("Bridge PDA: {}", bridge_pda);
    println!("User PDA: {}", user_balance_pda);  
    println!("User pubkey: {}", user_pubkey);
    println!("Relayer pubkey: {}", complete_keypair.pubkey());
    println!("🔓 Unlocking {} SOL for {} (Solana address: {})", amount, user, solana_address);
    

    // Wrap it in a transaction
    // ✅ Get the latest blockhash
    let recent_blockhash = match rpc_client.get_latest_blockhash(){
        Ok(recent) => {
            println!("[RECENT HASH]: {}", recent);
            recent
        }
        Err(e)=>{
            println!("Hash Error {}", e);
            return Ok(())
        }
    };


    println!("The Recent Block Hash {}:", recent_blockhash);

    let tx = Transaction::new_signed_with_payer(
        &[my_instruction],
        Some(&complete_keypair.pubkey()),
        &[&complete_keypair],
        recent_blockhash,
    );
    //println!("TRANSACTION {:?}", tx);


    // Send the transaction
    let signature = match rpc_client.send_and_confirm_transaction(&tx){
        Ok(signature) => {
            println!("[SIGNATURE]: {}", signature);
            signature
        }
        Err(e)=>{
            println!("signer_confirm Error {}", e);
            return Ok(())
        }
    };
    println!("✅ Unlock successful! Tx Signature: {}", signature);
    
    Ok(())
}

fn calculate_anchor_discriminator(instruction_name : &str)->[u8; 8]{
    let namespace = "global";
    let full_name = format!("{}:{}",namespace, instruction_name );

    let mut hasher = Sha256::new();
    hasher.update(full_name.as_bytes());
    let hash = hasher.finalize();

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash[..8]);
    discriminator

}