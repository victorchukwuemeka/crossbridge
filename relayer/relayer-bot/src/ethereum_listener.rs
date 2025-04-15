use std::time;

use std::error::Error;
//use tokio::time;

pub async fn start()->Result<(), Box<dyn Error>>{
    println!(" listening to ethereum LockEvent ");
    tokio::time::sleep(time::Duration::from_secs(5)).await;
    println!("Solana LockEvent handled!");
    Ok(())
} 