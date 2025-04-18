
use std::error::Error;

mod solana_listener;
mod ethereum_listener;
mod ethereum_minter;




#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    
    println!("Relayer bot starting up...");
    tokio::try_join!(
        solana_listener::start(),
        ethereum_listener::start(),
    )?;
    
    Ok(())
}
