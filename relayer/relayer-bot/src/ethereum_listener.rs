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

use web3::types::U256; 

// Minimal type definitions
type Address = [u8; 20];  // Ethereum address
type Hash = [u8; 32];     // For topics/hashes
//type U256 = [u8; 32];     // 256-bit unsigned integer


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
    keccak256("Burned(address,uint256,string)".as_bytes())
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
    /*let contract_addresss:Address  = hex::decode(
        env::var("WSOL_CONTRACT_ADDRESS")?
        .trim_start_matches("0x")
    )?
    .try_into()
    .map_err(|_| "Invalid address length")?;*/
    
    let env_addr = match env::var("WSOL_CONTRACT_ADDRESS"){
        Ok(addr)  => addr,
        Err(e) => return Err(e.into())
    };

    let decode = match hex::decode(env_addr.trim_start_matches("0x")){
        Ok(decode) => decode,
        Err(e) => return Err(e.into())
    };

    let contract_address:Address = match decode.try_into(){
        Ok(contract) => contract,
        Err(_) => return Err("invalid address length".into())
    };




    println!("The Contract address: {:?}.",contract_address );
    println!("Contract address hex: 0x{}", hex::encode(contract_address));

    // Validate it's 20 bytes
    if contract_address.len() != 20 {
        return Err(format!("Invalid contract address length: {}", contract_address.len()).into());
    }

    
    let event_topic = get_custom_burn_event_signature_hash();
    println!("The topic we are Looking for in the contract: {:?}.", event_topic);

    
    println!("Contract address hex: 0x{}", hex::encode(contract_address));
    
    loop {
          
        //get current block
        let current_block = match get_current_block_number(&client).await{
            Ok(block) => {
                println!("Current Block BBBBBBBBBBBB{}", block);
                block
            }
            Err(e)=> return Err(e.into())
        };
      
        //let from_block = ; // Last 100 blocks
        let temp_last_block = current_block - 50;
        let temp_current_block  =  current_block ;

        // Fetch logs
        let logs = match get_logs_in_range(
            &client,
            contract_address,
            event_topic,
            temp_last_block  ,
            temp_current_block
          // current_block - 100,
          // current_block,
           
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
            
            println!("let run the log");
            let log_data = match parse_burn_log(&log){
                Ok(log) => log,
                Err(e) => return Err(e.into())
            };
            println!("this is the LOG Data {:?}", log_data);
            
            if let Ok((user, amount, solana_address,tx_hash )) = parse_burn_log(&log) {
                println!("Parsed event - User: {:?}, Amount: {:?}, and Solana Address:{}", hex::encode(user), amount, solana_address);
                
                match solana_unlocker::unlock(
                    hex::encode(user),
                    amount,
                    solana_address.to_string(),
                 tx_hash.to_string()
                ).await{
                    Ok(solana) => solana,
                    Err(e)=> return Err(e.into())
                };
            }else {
                println!("i did not get the User And Amount Right");
            }
        }
        
        //last_block = current_block;
        tokio::time::sleep(time::Duration::from_secs(5)).await;
    }
}


/*async fn get_block_number(client: &HttpClient) -> Result<u64, Box<dyn Error>> {
    let response: Value = client.request("eth_blockNumber", Vec::<()>::new()).await?;
    let hex_string = response.as_str().ok_or("Response is not a string")?;
    let hex_without_prefix = hex_string.trim_start_matches("0x");
    let block_number = u64::from_str_radix(hex_without_prefix, 16)?;

    Ok(block_number)
}*/


async fn get_current_block_number(client: &HttpClient) -> Result<u64, Box<dyn Error>> {
    let response: Value = match client.request("eth_blockNumber", rpc_params![]).await{
        Ok(request) => request,
        Err(e) => return Err(e.into())
    };
    let hex_block = match response.as_str().ok_or("Invalid block number response"){
        Ok(block_response) => block_response,
        Err(e) => return Err(e.into())
    };
    let result = match u64::from_str_radix(&hex_block[2..], 16){
        Ok(radix) => radix,
        Err(e) => return Err(e.into())
    };
    Ok(result)
}

async fn get_logs_in_range(
    client: &HttpClient,
    address: Address,
    topic: Hash,
    from_block: u64,
    to_block: u64,
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
    println!("   address: 0x{}", hex::encode(address));
    println!("   topic: 0x{}", hex::encode(topic));
    // println!("   Full param JSON: {}", serde_json::to_string_pretty(params).unwrap());
    
    let response: Value = match client.request("eth_getLogs",  rpc_params![filter]).await{
        Ok(request)=> {
            println!("Response {}", request);
            request
        },
        Err(e) => return Err(e.into())
    };
    Ok(response.as_array().cloned().unwrap_or_default())
}


/*async fn check_contract_exists(client: &HttpClient, address: &Address) -> Result<(), Box<dyn Error>> {
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
}*/



/*fn parse_burn_log(log: &Value) -> Result<(Address, U256, String, String), Box<dyn Error>> {
    let topics = log["topics"].as_array().ok_or("Missing topics")?;
    let data = log["data"].as_str().ok_or("Missing data")?;
    
    let tx_hash = log["transactionHash"].as_str()
        .ok_or("Missing transaction hash")?
        .to_string();


    // Parse user address (topic 1)
    let user_topic = topics[1].as_str().ok_or("Missing topic")?;
    let user_bytes = hex::decode(&user_topic[26..])?;
    let user: Address = user_bytes.try_into().map_err(|_| "Invalid address")?;

    let data_bytes = hex::decode(&data[2..])?;

    if data_bytes.len() < 96 {
        return Err("Data too short".into());
    }

    // Parse amount (first 32 bytes)
    let amount_bytes: [u8; 32] = data_bytes[0..32].try_into()?;
    let amount = U256::from_big_endian(&amount_bytes);

    // Parse string length (bytes 64-95, last 4 bytes)
    let string_length = u32::from_be_bytes([
        data_bytes[92], data_bytes[93], data_bytes[94], data_bytes[95]
    ]) as usize;

    // Parse solana address (starting from byte 96)
    if data_bytes.len() < 96 + string_length {
        return Err("Data too short for string".into());
    }
    
    let solana_address = String::from_utf8(
        data_bytes[96..96 + string_length].to_vec()
    )?;

    println!("User: {:?}", user);
    println!("Amount: {:?}", amount);
    println!("Solana Address: {}", solana_address);
    println!("Transaction Hash: {}", tx_hash);
    
    Ok((user, amount, solana_address,tx_hash))
}*/



fn parse_burn_log(log: &Value) -> Result<(Address, f64, String, String), Box<dyn Error>> {
    let topics = match log["topics"].as_array(){
        Some(topics_log ) => topics_log,
        None => {
            return Err("invalide topics".into())
        }
    };
    let data = match log["data"].as_str(){
        Some(data) => data,
        None => {
            return Err("Invalid data".into())
        }
    };
   /*let tx_hash = match log["transactionHash"]
        .as_str()
        .or
        .to_string(){
            Some(txhash) => txhash,
            None => {
                return Err("Invalide txhash".into())
            }
        };*/
    let tx_hash = match log["transactionHash"].as_str() {
        Some(txhash) => {
            println!("✅ Found transaction hash");
            txhash.to_string()  
        }
        None => {
            println!("❌ Missing transaction hash");
            return Err("Invalid txhash".into());
        }
    };

    // Parse user address (topic 1) - sender of the burn
    let user_topic = match topics[1].as_str(){
        Some(topic) => topic,
        None => return Err("Invalid topic".into())
    };
    
    let user_bytes = hex::decode(&user_topic[26..])?; // Strip 0x and 24 bytes of padding
    let user: Address = user_bytes.try_into().map_err(|_| "Invalid address")?;

    // Parse amount (from data field)
    let data_bytes = hex::decode(&data[2..])?; // Remove 0x prefix
    
    // The amount is the first 32 bytes of data in big-endian
    let amount_bytes: [u8; 32] = data_bytes[0..32].try_into()?;
    let amount = U256::from_big_endian(&amount_bytes);

    // Convert raw amount to 0.5 wSOL (assuming 9 decimals like Solana)
    let decimals = 9;
    let human_amount = amount.as_u128() as f64 / 10_u64.pow(decimals) as f64;
    println!("Amount burned: {} wSOL", human_amount); // Should print "0.5"

    // Parse Solana address (dynamic string at byte 96)
    let string_len = u32::from_be_bytes([data_bytes[92], data_bytes[93], data_bytes[94], data_bytes[95]]) as usize;
    if data_bytes.len() < 96 + string_len {
        return Err("Data too short for Solana address".into());
    }
    let solana_address = String::from_utf8(data_bytes[96..96 + string_len].to_vec())?;
     
    Ok((user, human_amount, solana_address, tx_hash))
}


/*fn hex_to_u64(value: &Value) -> u64 {
    u64::from_str_radix(value.as_str().unwrap().trim_start_matches("0x"), 16).unwrap_or(0)
}

fn bytes_to_u256(bytes: &[u8; 32]) -> u128 {
    bytes.iter().fold(0u128, |acc, &byte| (acc << 8) | byte as u128)
}*/