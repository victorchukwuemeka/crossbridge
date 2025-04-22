use std::{error::Error, fs::File, str::FromStr, rc::Rc};

use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::{Keypair, Signer};

use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_config::{RpcTransactionLogsConfig,RpcTransactionLogsFilter,};
use solana_client::rpc_response::{RpcLogsResponse, Response};

use solana_sdk::pubkey::Pubkey;
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

use serde_json;
use tokio::time::Duration;
//use tokio_stream::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::stream::StreamExt;





pub async fn start() -> Result<(), Box<dyn Error>> {
    // Load key for Solana config part
    let key_pair = "/home/victor/.config/solana/id.json";
    let file = File::open(key_pair)?;
    let keypair_bytes: Vec<u8> = serde_json::from_reader(file)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;

    // Network devnet for now
    let client = Client::new_with_options(
        Cluster::Custom(
            "http://127.0.0.1:8899".to_string(), // RPC URL
            "ws://127.0.0.1:8900".to_string()    // WebSocket URL
        ),
        Rc::new(keypair),
        CommitmentConfig::processed(),
    );

    // Initialize WebSocket client
    let websocket_url = "ws://127.0.0.1:8900"; 
    let (mut ws_client, _) = connect_async(websocket_url).await?;
   

    println!("Solana listener started ...!");

    
    let program_id = Pubkey::from_str("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM").unwrap();
    let program =  client.program(program_id);
    
    let pubsub_client = PubsubClient::new(websocket_url).await?;
   

    
    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        }),
    };

    let filter = RpcTransactionLogsFilter::Mentions(vec![program_id.to_string()]);

    let (mut logs_subscription, _unsubscribe) = pubsub_client.logs_subscribe(filter, config).await?;

    

    while let Some(log_response) = logs_subscription.next().await{
        handle_log_response(log_response).await?;
    }
   
    Ok(())
}



async fn handle_log_response(
    log_response: Response<RpcLogsResponse>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Log Response: {:?}", log_response.value);

    let logs = log_response.value.logs ;
    
    for log in logs {
        println!("🔍 Log: {}", log);
        if log.contains("LockEvent") {
            println!("✅ LockEvent found!");

                
            let user = "0xFABB0ac9d68B0B445fB7357272Ff202C5651694a";
            let amount = 1000;

            // Mint tokens on Ethereum side
            crate::ethereum_minter::mint_wsol(user, amount).await?;
        }
    }
   
    tokio::time::sleep(Duration::from_secs(3)).await; // Sleep for 3 seconds
    println!("Listening to Event...");
    Ok(())
}
