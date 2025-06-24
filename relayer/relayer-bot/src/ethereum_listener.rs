use std::time;
use std::error::Error;
use web3::transports::Http;
use web3::api::Web3;
use web3::types::{FilterBuilder, Log, H160, U256, BlockNumber};
use web3::types::Address;
use crate::solana_unlocker;
use std::env;

pub async fn start() -> Result<(), Box<dyn Error>> {
    let eth_devnet_rpc_url = env::var("ETH_DEVNET_RPC_URL")
        .unwrap_or("https://eth-sepolia.g.alchemy.com".into());
    let transport = Http::new(&eth_devnet_rpc_url)?;
    let web3 = Web3::new(transport);
    
    let contract_address: Address = env::var("WSOL_CONTRACT_ADDRESS")?.parse()?;
    let mut last_block = web3.eth().block_number().await?;
    
    loop {
        let current_block = web3.eth().block_number().await?;
        
        if current_block > last_block {
            let filter = FilterBuilder::default()
                .address(vec![contract_address])
                .from_block(BlockNumber::Number(last_block + 1))
                .to_block(BlockNumber::Number(current_block))
                .topics(Some(vec!["0xYourBurnEventSignatureHash".parse().unwrap()]), None, None, None)
                .build();
            
            let logs = web3.eth().logs(filter).await?; // This returns Vec<Log>, not a stream
            
            for log in logs {
                println!("Event log received");
                let (user, amount, solana_address) = parse_burn_event(log)?;
                solana_unlocker::unlock(format!("{:?}", user), amount.as_u64(), solana_address).await?;
            }
            
            last_block = current_block;
        }
        
        tokio::time::sleep(time::Duration::from_secs(5)).await;
    }
}

fn parse_burn_event(log: Log) -> Result<(H160, U256, String), Box<dyn std::error::Error>> {
    let data = log.data.0.clone();
    let user = H160::from_slice(&data[0..20]);
    let amount = U256::from_big_endian(&data[20..52]);
    let solana_addr = String::from_utf8(data[52..].to_vec())?;
    Ok((user, amount, solana_addr))
}