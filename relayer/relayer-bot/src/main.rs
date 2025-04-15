
use std::error::Error;

mod solana_listener;
mod ethereum_listener;




#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    // async code here
    tokio::try_join!(
        solana_listener::start(),
        ethereum_listener::start(),
    )?;
    println!("Relayer bot starting up...");
    Ok(())
}
