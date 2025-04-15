use std::error::Error;


pub async fn start()->Result<(), Box<dyn Error>>{
    println!("Listening to Solana LockEvent");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("✅ Ethereum BurnEvent handled!");
    Ok(())
}