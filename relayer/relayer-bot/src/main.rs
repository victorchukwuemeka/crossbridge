use std::error::Error;
mod solana_listener;
mod ethereum_listener;
mod ethereum_minter;
mod solana_unlocker;
mod burn_tracker;
mod base_minters;
mod utils;
mod fetch_tx_and_block_header;
mod merkle;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Relayer bot starting up...");
    
    dotenv::dotenv().ok(); 
    tokio::try_join!(
        solana_listener::start(),
        ethereum_listener::start(),
    )?;
    
    Ok(())
}