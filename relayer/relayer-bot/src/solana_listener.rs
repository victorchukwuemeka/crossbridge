use std::{error::Error, str::FromStr};
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    rpc_response::{Response, RpcLogsResponse},
};
use solana_sdk::{pubkey::Pubkey, commitment_config::{CommitmentConfig, CommitmentLevel}};
use tokio::time::Duration;
use futures::StreamExt;

pub async fn start() -> Result<(), Box<dyn Error>> {
    // Devnet WebSocket endpoint
    let websocket_url = "wss://api.devnet.solana.com";
    let pubsub_client = PubsubClient::new(websocket_url).await?;

    println!("Solana Devnet listener started...!");

    let program_id = Pubkey::from_str("911VdUg43JGvomS2eCqKHJcUZ6J9SCjb371w6Xst7YMD")?;

    let config = RpcTransactionLogsConfig {
        commitment: Some(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        }),
    };

    let filter = RpcTransactionLogsFilter::Mentions(vec![program_id.to_string()]);
    let (mut logs_subscription, _unsubscribe) = pubsub_client.logs_subscribe(filter, config).await?;

    while let Some(log_response) = logs_subscription.next().await {
        handle_log_response(log_response).await?;
    }

    Ok(())
}

async fn handle_log_response(
    log_response: Response<RpcLogsResponse>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("📥 Log Response: {:?}", log_response.value);

    for log in log_response.value.logs {
        println!("🔍 Log: {}", log);
        if log.contains("LockEvent") {
            println!("✅ LockEvent found!");
            let user = "0xFABB0ac9d68B0B445fB7357272Ff202C5651694a";
            let amount = 1000;
            // Mint tokens on Ethereum side
            crate::ethereum_minter::mint_wsol(user, amount).await?;
        }
    }
    
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("Listening to Event...");
    Ok(())
}