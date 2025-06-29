use std::time;
use std::error::Error;
use jsonrpsee::http_client::{HttpClientBuilder, HttpClient};
//use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::types::Params;
use serde_json::{json, Value};
use web3::types::U64;
use std::env;
use hex; // For hex decoding
//use jsonrpsee::core::client::params::RpcParams;
//use jsonrpsee::types::params::ParamsSer;
//use jsonrpsee::types::Params;
use jsonrpsee::rpc_params;
use crate::solana_unlocker;

// Minimal type definitions
type Address = [u8; 20];  // Ethereum address
type Hash = [u8; 32];     // For topics/hashes
type U256 = [u8; 32];     // 256-bit unsigned integer


// Calculate event signature hash (replaces alloy/ethers keccak256)
fn keccak256(data: &[u8]) -> Hash {
    use tiny_keccak::{Keccak, Hasher};
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}

pub fn get_custom_burn_event_signature_hash() -> Hash {
    keccak256("Burned(address,uint256)".as_bytes())
}


pub async fn start() -> Result<(), Box<dyn Error>> {
    println!("Starting Ethereum listener (minimal JSON-RPC version)");
    
    // Setup HTTP client
    let rpc_url = env::var("ETH_DEVNET_RPC_URL")
        .unwrap_or("https://eth-sepolia.public.blastapi.io".into());
    println!("Connected To This  Ethereum RPC: {}", rpc_url);
    //let client = match 
    //the client builder returns the actual client .
    let client  = match HttpClientBuilder::default().build(&rpc_url) {
        Ok(client )=>{
            println!("Fetched {:?} logs", client);
            client
        }
        Err(e)=> {
            println!("Log fetch failed: {}", e);
            return Err(e.into()); 
        }
    };

    println!("Client builder : {:?}", client);
    
    // Parse contract address and validate if is valide 
    let contract_address:Address  = hex::decode(
        env::var("WSOL_CONTRACT_ADDRESS")?.trim_start_matches("0x")
    )?.try_into().map_err(|_| "Invalid address length")?;
    println!("The Contract address: {:?}.",contract_address );
    println!("Contract address hex: 0x{}", hex::encode(contract_address));

    // Validate it's 20 bytes
    if contract_address.len() != 20 {
        return Err(format!("Invalid contract address length: {}", contract_address.len()).into());
    }

    
    let event_topic = get_custom_burn_event_signature_hash();
    println!("The topic we are Looking for in the contract: {:?}.", event_topic);

    let mut last_block = match get_block_number(&client).await{
        Ok(last_block)=>{
            println!("We Have Our Last Block :{}.", last_block);
             last_block.saturating_sub(1) 
        }
        Err(e)=>{
            println!("We Failed To Get The Last Block: {}", e);
            return Err(e.into()); 
        }
    };
    //?.saturating_sub(1);
    //println!("Starting from block: {}", last_block);
    println!("Contract address hex: 0x{}", hex::encode(contract_address));
    
    loop {
       // let current_block = get_block_number(&client).await?;
        
       // println!("Checking blocks {} to {}", last_block + 1, current_block);
        let temp_last_block  = 8625100;
        let temp_current_block = 8625100;

        // Fetch logs
        let logs = match get_logs_in_range(
            &client,
            contract_address,
            event_topic,
           // last_block + 1,
           // current_block
           temp_last_block-10,
           temp_current_block+10
           
        ).await {
            Ok(logs) => {
                println!("Fetched {:?} logs", logs);
                logs
            },
            Err(e) => {
                eprintln!("Log fetch failed: {:?}", e);
                // Fallback: try just the latest block
                println!("Trying fallback (latest block only)...");
                get_logs_in_range(
                    &client,
                    contract_address,
                    event_topic,
                    temp_last_block,
                    temp_current_block
                ).await.unwrap_or_default()
            }
        };
        
        // Process logs
        for log in logs {
            if let Ok((user, amount)) = parse_burn_log(&log) {
                println!("Parsed event - User: {:?}, Amount: {}", hex::encode(user), bytes_to_u256(&amount));
               //solana_unlocker::unlock(hex::encode(user), bytes_to_u256(&amount)).await?;
            }
        }
        
        //last_block = current_block;
        tokio::time::sleep(time::Duration::from_secs(5)).await;
    }
}


async fn get_block_number(client: &HttpClient) -> Result<u64, Box<dyn Error>> {
    let response: Value = client.request("eth_blockNumber", Vec::<()>::new()).await?;
    let hex_string = response.as_str().ok_or("Response is not a string")?;
    let hex_without_prefix = hex_string.trim_start_matches("0x");
    let block_number = u64::from_str_radix(hex_without_prefix, 16)?;

    Ok(block_number)
}


async fn get_logs_in_range(
    client: &HttpClient,
    address: Address,
    topic: Hash,
    from_block: u64,
    to_block: u64
) -> Result<Vec<Value>, Box<dyn Error>> {
    

    let filter = json!({
        "address": format!("0x{}", hex::encode(address)),
        "fromBlock": format!("0x{:x}", from_block),
        "toBlock": format!("0x{:x}", to_block),
        "topics": [format!("0x{}", hex::encode(topic))]
    });

    println!("🔍 DEBUG - Filter being sent:");
    println!("   fromBlock: 0x{:x}", from_block);
    println!("   toBlock: 0x{:x}", to_block);
    //println!("   address: 0x{}", hex::encode(address));
    //println!("   topic: 0x{}", hex::encode(topic));
    // println!("   Full param JSON: {}", serde_json::to_string_pretty(params).unwrap());
    
    let response: Value = client.request("eth_getLogs",  rpc_params![filter]).await?;
    println!("Response {}", response);
    Ok(response.as_array().cloned().unwrap_or_default())
}


async fn check_contract_exists(client: &HttpClient, address: &Address) -> Result<(), Box<dyn Error>> {
    let address_hex = format!("0x{}", hex::encode(address));
    println!("🔍 Checking if contract exists at: {}", address_hex);
    
    // Get contract code
    let code_response: Value = client.request("eth_getCode", vec![
        json!(address_hex),
        json!("latest")
    ]).await?;
    
    if let Some(code) = code_response.as_str() {
        if code == "0x" || code.is_empty() {
            println!("❌ No contract found at address {}", address_hex);
        } else {
            println!("✅ Contract exists! Code length: {}", code.len());
        }
    }
    
    Ok(())
}



fn parse_burn_log(log: &Value) -> Result<(Address, U256), Box<dyn Error>> {
    let topics = log["topics"].as_array().ok_or("Missing topics")?;
    let data = log["data"].as_str().ok_or("Missing data")?;
    
    // Parse user address (topic 1)
    let user = hex::decode(&topics[1].as_str().unwrap()[2..])?
        .try_into()
        .map_err(|_| "Invalid address length")?;
    
    // Parse amount (first 32 bytes of data)
    let amount = hex::decode(&data[2..66])? // Skip '0x', take first 64 hex chars
        .try_into()
        .map_err(|_| "Invalid U256 length")?;
    
    Ok((user, amount))
}

fn hex_to_u64(value: &Value) -> u64 {
    u64::from_str_radix(value.as_str().unwrap().trim_start_matches("0x"), 16).unwrap_or(0)
}

fn bytes_to_u256(bytes: &[u8; 32]) -> u128 {
    bytes.iter().fold(0u128, |acc, &byte| (acc << 8) | byte as u128)
}