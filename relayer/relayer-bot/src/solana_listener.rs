use std::{error::Error, str::FromStr};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, commitment_config::CommitmentConfig};
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use tokio::time::{Duration, sleep};

use borsh::{BorshDeserialize};

#[derive(Debug, BorshDeserialize)]
pub struct LockEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}





pub async fn start() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://api.devnet.solana.com";
   // let rpc_url = "https://devnet.solana.com";
    //let rpc_url = "https://devnet.genesysgo.net";
    //let rpc_url = "https://devnet.helius-rpc.com/?api-key=29074803-1dba-4c7c-94c9-9ee93d78de8f";
    let client = RpcClient::new(rpc_url.to_string());
    

    // Test the connection first
    println!("Testing connection...");
    match client.get_health() {
        Ok(health) => println!("✅ Connection successful: {:?}", health),
        Err(e) => {
            println!("❌ Connection test failed: {:?}", e);
            return Err(e.into());
        }
    }
    

    println!("✅ Connected to Solana Devnet: {}", rpc_url);

    println!("Solana Devnet listener started...!");
    
    let program_id = Pubkey::from_str("8aHKSiDSTpVZQBm6HZSmUJSJPxAFPjCMybuEns6Lbn5a")?;
    
    loop {


        // Get recent transactions for the program
        let signatures = match  client.get_signatures_for_address(&program_id){
            Ok(sigs) => {
                println!("✅ Found {} signatures", sigs.len());
                sigs
            }
            Err(e) => {
                println!("❌ Error getting signatures: {:?}", e);
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        if signatures.is_empty() {
            println!("ℹ️ No transactions found for this program yet");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        
        for sig_info in signatures.iter().take(5) { // Check last 5 transactions
            let signature = solana_sdk::signature::Signature::from_str(&sig_info.signature)?;
            
            // Get transaction details
            if let Ok(transaction) = client.get_transaction(&signature, UiTransactionEncoding::Json) {
                //println!("SHOW TRANSACTION  AS {:?} :", transaction);
                if let Some(meta) = transaction.transaction.meta {
                    let logs = match &meta.log_messages {
                        OptionSerializer::Some(logs) => logs.clone(),
                        OptionSerializer::None => continue,
                        OptionSerializer::Skip => continue,
                    };
                    handle_logs(&sig_info.signature, logs).await?;
                }
            }
        }
        
        sleep(Duration::from_secs(10)).await;
        println!("🔄 Checking for new transactions...");
    }
}


async fn handle_logs(signature: &str, logs: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("\n=== Processing transaction: {} ===", signature);
    println!("Total logs found: {}", logs.len());
    
    for (i, log) in logs.iter().enumerate() {
        println!("\n[Log {}]: {}", i, log);
        
        // Check for serialized event data (base64 encoded)
        if log.starts_with("Program data:") {
            let data = &log["Program data: ".len()..];
            println!("🔍 Found serialized event data (base64): {}", data);
            
            match base64::decode(data) {
                Ok(decoded) => {
                    println!("✅ Successfully decoded base64 ({} bytes)", decoded.len());
                    
                    // Try to interpret as UTF-8 string first
                    match String::from_utf8(decoded.clone()) {
                        Ok(string) => {
                            println!("📝 UTF-8 decoded: {}", string);
                            if string.contains("LockEvent") {
                                println!("🎉 FOUND LockEvent in string data!");

                            }

                        },
                        Err(_) => println!("⚠️ Data is not UTF-8 text (likely binary format)"),
                    }
                    
                    // Hex dump for binary data
                    let binary_hex = hex::encode(&decoded);
                    println!("🔢 Hex dump: {}", &binary_hex);

                    
                    // my custom event deserialization 
                    match LockEvent::try_from_slice(&decoded[8..]){
                        Ok(event)=>{
                             println!("🎉 ✅ LockEvent found in tx: {}", signature);
                             println!("   User: {}", event.user);
                             println!("   Amount: {}", event.amount);
                             println!("   Timestamp: {} ({})", event.timestamp, 
                                chrono::DateTime::from_timestamp(event.timestamp, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "Invalid timestamp".to_string()));

                            let user = "hhhhf";
                            //event.user;
                            let amount = event.amount;

                            
                            // Mint tokens on Ethereum side
                           // crate::ethereum_minter::mint_wsol(user, amount).await?;
                        },
                         Err(_) => println!("⚠️ Found program data, but not a LockEvent"),
                    }
                },
                Err(e) => println!("❌ Failed to decode base64: {}", e),
            }
        }
        // Check for plain text logs
        else if log.contains("LockEvent") {
            println!("🎉 FOUND LockEvent in plain text log!");
        }
    }
    
    println!("=== End of transaction {} ===", signature);
    Ok(())
}