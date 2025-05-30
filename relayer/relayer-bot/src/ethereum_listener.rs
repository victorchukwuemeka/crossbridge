use std::time;
use std::error::Error;
use web3::transports::ws::WebSocket;
use web3::api::Web3;
use web3::types::{FilterBuilder, Log, H160, U256};
use web3::types::Address;
use futures::stream::StreamExt; // For next() method
use crate::solana_unlocker;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let web_socket = WebSocket::new("ws://127.0.0.1:8545").await?;
    let web3 = Web3::new(web_socket);
    
    let contract_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse()?;
    
    let filter = FilterBuilder::default()
        .address(vec![contract_address])
        .topics(
            Some(vec!["0xYourBurnEventSignatureHash".parse().unwrap()]),
            None, None, None
        )
        .build();
    
    let mut sub = web3.eth_subscribe().subscribe_logs(filter).await?;
    
    while let Some(log_result) = sub.next().await {
        match log_result {
            Ok(log) => {
                println!("Event log received");
                let (user, amount, solana_address) = parse_burn_event(log)?;
                solana_unlocker::unlock(format!("{:?}", user), amount.as_u64(), solana_address).await?
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    
    tokio::time::sleep(time::Duration::from_secs(5)).await;
    Ok(())
}

fn parse_burn_event(log: Log) -> Result<(H160, U256, String), Box<dyn std::error::Error>> {
    let data = log.data.0.clone();
    
    let user = H160::from_slice(&data[0..20]);
    let amount = U256::from_big_endian(&data[20..52]);
    let solana_addr = String::from_utf8(data[52..].to_vec())?;
    
    Ok((user, amount, solana_addr))
}