use std::error::Error;
use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use solana_sdk::commitment_config::CommitmentConfig;
use std::fs::File;
use std::rc::Rc;
use serde_json;
use tokio::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};



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

    let program_id = client.program("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM");

    loop {
        // You would normally use WebSocket or RPC polling
        // Polling recent transactions and filtering for LockEvent
        match ws_client.subscribe_log(program_id).await {
            Ok(log) => {
                println!("Received log {:?}", log);

                if let Some(log_data) = log {
                    if log_data.contains("LockEvent") {
                        println!("LockEvent detected: {:?}", log_data);

                        let user = "0x...";  // Replace with actual user address
                        let amount = 1000;

                        // Call Ethereum minter
                        crate::ethereum_minter::mint_wsol(user, amount).await?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving log: {}", e);
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await; // Sleep for 3 seconds
        println!("Listening to Event...");
    }

    Ok(())
}
