use std::{error::Error, str::FromStr, collections::HashSet};
use serde::de::value;
use solana_client::rpc_client::{self, RpcClient};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature};
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use tokio::time::{Duration, sleep};
use crate::solana_state_client::SolanaStateClient;


use borsh::{BorshDeserialize};

#[derive(Debug, BorshDeserialize)]
pub struct LockEvent {
    pub user: Pubkey,
    pub eth_address: String,
    pub amount: u64,
    pub fees: u64,
    pub target_network: u8,
    pub timestamp: i64,
}


//for listening to solana across functions
pub struct ListenerContext {
    pub rpc_url: String,
    pub program_id: Pubkey,
    pub state_client: SolanaStateClient,
}

impl ListenerContext {
    pub fn new(rpc_url: String, program_id:Pubkey)->Self{
        let state_client  = SolanaStateClient::new(rpc_url.clone(), program_id);
        Self { rpc_url, program_id, state_client }
    }

}

#[derive(Debug, Clone, Copy)]
pub enum TargetNetwork{
    Ethereum = 1,
    Base = 121,
    Polygon = 137
}

impl From<u8> for TargetNetwork  {
    fn from(value: u8) -> Self {
        match value {
            1 => TargetNetwork::Ethereum,
            121 => TargetNetwork::Base,
            137 => TargetNetwork::Polygon,
            _ => panic!("Invalid network"),
        }
    }
    
}




//  to track processed signatures
static mut PROCESSED_SIGNATURES: Option<HashSet<String>> = None;
const PROCESSED_FILE: &str = "processed_sigs.txt";



fn get_processed_signatures() -> &'static mut HashSet<String> {
    unsafe {
        PROCESSED_SIGNATURES.get_or_insert_with(||  {
            load_processed_signature().unwrap_or_else(|_| HashSet::new())
        })
    }
}

fn load_processed_signature()->Result<HashSet<String>, Box<dyn Error>>{
    if std::path::Path::new(PROCESSED_FILE).exists() {
        let data = match std::fs::read_to_string(PROCESSED_FILE){
            Ok(data) => {
                //println!("[DATA FROM THE PROCESSED FILE]: {}", data);
                data
            },
            Err(e)=> {
                println!("[DATA NOT GOTTEN] : {}", e);
                return Ok(HashSet::new());
            }
        };
        let signatures: HashSet<String> = match serde_json::from_str(&data){
            Ok(sign )=> {
               // println!("[SIGN INFO OF JSON]:{:?}", sign);
                sign
            }
            Err(e)=> {
                println!("[DATA IN JSON FORM FAILED]: {}",e);
                return Ok(HashSet::new());
            }
        };
        println!("📁 Loaded {} processed signatures from file", signatures.len());
        Ok(signatures)
    }else {
        println!("📁 No existing processed signatures file found, starting fresh");
        Ok(HashSet::new())
    }
}

fn save_processed_signatures()->Result<(), Box<dyn Error>>{
    let signatures = get_processed_signatures();
    let data = match serde_json::to_string(signatures){
        Ok(data) => {
           // println!("[DATA GOTTEN]:{:?}",data);
            data
        },

        Err(e)=>{
            println!("[DATA LOST] : {}", e);
            return Ok(());
        }
    };
    match std::fs::write(PROCESSED_FILE, data){
        Ok(file)=> {
           // println!("[FILE WITH DATA]: {:?}", file);
            file
        }
        Err(e)=>{
            println!("No Data Processed {}", e);
            return Ok(());
        }
    };

    println!("💾 Saved {} processed signatures to file", signatures.len());
    Ok(())
}







//1. get the solana api
//2. use the solana api to get rpcClient.
//3. check if  the connection with the rpc client is valid
//4. get the program id
//5. get the signature to recent transactions
//6. check if the transactions are empty.
//7. get the transaction details 
//8. return handle logs. 
pub async fn start() -> Result<(), Box<dyn Error>> {
    let rpc_url = "https://api.devnet.solana.com";
   // let rpc_url = "https://devnet.solana.com";
    //let rpc_url = "https://devnet.genesysgo.net";
    //let rpc_url = "https://devnet.helius-rpc.com/?api-key=29074803-1dba-4c7c-94c9-9ee93d78de8f";
    let client = RpcClient::new(rpc_url.to_string());
    

    // Test the connection first
    println!("Testing connection...");
    match client.get_health() {
        Ok(health) => println!("✅ Connection successful: {:?}", health),
        Err(e) => {
            println!("❌ Connection test failed: {:?}", e);
            return Err(e.into());
        }
    }
    

    println!("✅ Connected to Solana Devnet: {}", rpc_url);

    println!("Solana Devnet listener started...!");
    
    let program_id = match Pubkey::from_str("7N9UCyKUqac5JuEjn4inZcBFhi87FXDRy3rP1mNhTrdB"){
        Ok(program )=>{
            //println!("[PROGRAM ID] : {}", program);
            program
        }
        Err(e)=>{
            println!("Could not find  program id {}",e);
            return Ok(());
        }
    };

    //making it easy to pass rpc data between functions
    let ctx = ListenerContext::new(rpc_url.to_string(), program_id);
    
    loop {

        // Get recent transactions signatures using  the program id 
        let signatures = match  client.get_signatures_for_address(&program_id){
            Ok(sigs) => {
               // println!("✅ Found {} signatures", sigs.len());
                sigs
            }
            Err(e) => {
                println!("❌ Error getting signatures: {:?}", e);
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };


        if signatures.is_empty() {
            println!("ℹ️ No transactions found for this program yet");
            sleep(Duration::from_secs(10)).await;
            continue;
        }

        
        for sig_info in signatures.iter().take(5) { // Check last 5 transactions

            //  to skip already processed transactions
            if get_processed_signatures().contains(&sig_info.signature) {
                continue;
            }

            let signature = match solana_sdk::signature::Signature::from_str(&sig_info.signature){
                Ok(sign) => {
                    println!("{} :[SIGNATURE GOTTEN]", sign);
                    sign
                },
                Err(e)=>{
                    println!("signature error:{}",e);
                    return Ok(());
                }
            };
            
            // Get transaction details
            if let Ok(transaction) = client.get_transaction(&signature, UiTransactionEncoding::Json) {
                //println!("SHOW TRANSACTION  AS {:?} :", transaction);
                if let Some(meta) = transaction.transaction.meta {
                    let logs = match &meta.log_messages {
                        OptionSerializer::Some(logs) => logs.clone(),
                        OptionSerializer::None => continue,
                        OptionSerializer::Skip => continue,
                    };
                    match handle_logs(&sig_info.signature, logs, &ctx).await{
                        Ok(handle) =>{
                            println!("[HANDLE LOGS]{:?}",handle);
                            handle
                        }
                        Err(e)=>{
                            println!("handle logs error: {}", e);
                            return Ok(());
                        }
                    };
                }
            }
        }
        
        sleep(Duration::from_secs(10)).await;
        println!("🔄 Checking for new transactions...");
    }
}


async fn handle_logs(signature: &str, logs: Vec<String>, ctx: &ListenerContext) -> Result<(), Box<dyn Error>> {
    println!("\n=== Processing transaction: {} ===", signature);
    println!("Total logs found: {}", logs.len());

    
    //the state client is used for pda verification before a transaction is made .
    let state_client = &ctx.state_client;
    
    
    for (i, log) in logs.iter().enumerate() {
        println!("\n[Log {}]: {}", i, log);
        
        // Check for serialized event data (base64 encoded)
        if log.starts_with("Program data:") {
            let data = &log["Program data: ".len()..];
            println!("🔍 Found serialized event data (base64): {}", data);
            
            /***
             * decode the serialized  event to understand the data 
             */
            match base64::decode(data) {
                Ok(decoded) => {
                    println!("✅ Successfully decoded base64 ({} bytes)", decoded.len());
                    
                    
                    /**
                     * convert the the vec<u8> to Utf-8  string
                     * check if the string has LockEvent
                     * if you and so just indicate 
                     */
                    match String::from_utf8(decoded.clone()) {
                        Ok(string) => {
                            println!("📝 UTF-8 decoded: {}", string);
                            if string.contains("LockEvent") {
                                println!("🎉 FOUND LockEvent in string data!");
                            }
                        },
                        Err(_) => println!("⚠️ Data is not UTF-8 text (likely binary format)"),
                    }
                    
                    // Hex dump for binary data
                    let binary_hex = hex::encode(&decoded);
                    println!("🔢 Hex dump: {}", &binary_hex);

                    
                    // my custom event deserialization 
                    /**
                     * at top of the LockEvent struct we have a macro 
                     * that we called in ther to help deserialize 
                     */
                    match LockEvent::try_from_slice(&decoded[8..]){
                        Ok(event)=>{
                             println!("🎉 ✅ LockEvent found in tx: {}", signature);
                             println!("   User: {}", event.user);
                             println!("  Eth_Address:{}", event.eth_address);
                             println!("   Amount: {}", event.amount);
                             println!(" fees: {}", event.fees);
                             println!(" target-network: {}", event.target_network);
                             println!("   Timestamp: {} ({})", event.timestamp, 
                                chrono::DateTime::from_timestamp(event.timestamp, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "Invalid timestamp".to_string()));

                            let user = event.user;
                            let amount = event.amount;
                            let eth_address= event.eth_address;
                            let target_network = event.target_network;
                            
                            /**
                             making sure the program state pda is matching with the solana program
                             so that the event data will 100  percent be the same with the program state
                            */
                            let lock_state = match state_client.get_user_lock_state(user){
                                Ok(Some(lock_state)) => lock_state,
                                Ok(None) => {
                                    println!(" No lock state found for user");
                                    return Ok(());
                                }
                                Err(e) => {
                                    println!(" Error retrieving lock state: {}", e);
                                    return Ok(());
                                }
                            };

                            println!("Lock State: {:?}", lock_state);

                            if lock_state.amount != amount{
                                
                                println!("❌ Amount mismatch! Event: {}, Lock State: {}", amount, lock_state.amount);
                                return Ok(());
                            }




                            let network = TargetNetwork::from(target_network);
                            match network{
                                TargetNetwork::Ethereum =>{
                                    
                                    // Mint tokens on Ethereum side
                                    match crate::ethereum_minter::mint_wsol(
                                        &user.to_string(), 
                                        amount,
                                        &eth_address,
                                        &signature.to_string()
                                    )
                                    .await{
                                        Ok(mint_ether) => {
                                            println!("[CALLING THE MINT ETHER MODEL]: {:?}", mint_ether);
                                            mint_ether
                                        }
                                        Err(e)=>{
                                            println!("[THE MINT ETHER CALL FAILED]: {}",e);
                                            return Ok(());
                                        }
                                    };
                                },

                                /**
                                 * selecting base network 
                                 */
                                TargetNetwork::Base =>{
                                    match crate::base_minters::mint_base_cwsol(
                                        &user.to_string(),
                                        amount,
                                        &eth_address,
                                        &signature.to_string()
                                    )
                                    .await{
                                        Ok(mint_base) => {
                                            println!("[CALLING THE MINT BASE MODEL]: {:?}", mint_base);
                                            mint_base
                                        }
                                        Err(e)=>{
                                            println!("[THE MINT BASE  CALL FAILED]: {}",e);
                                            return Ok(());
                                        }
                                    }
                                },

                                //
                                TargetNetwork::Polygon=> {

                                }
                            } 

                           

                            // Mark as processed only after successful minting
                           get_processed_signatures().insert(signature.to_string());
                           match save_processed_signatures(){
                            Ok(saved)=>{
                                //println!("[SAVED PROCESSED SIGNATURE]: {:?}", saved);
                                saved
                            }
                            Err(e)=>{
                                println!("failed to save processed signature");
                                return Ok(());
                            }
                           };
                           println!("✅ Transaction processed and marked: {}", signature);

                        },
                        Err(_) => println!("⚠️ Found program data, but not a LockEvent"),
                    }
                },
                Err(e) => println!("❌ Failed to decode base64: {}", e),
            }
        }
        // Check for plain text logs
        else if log.contains("LockEvent") {
            println!("🎉 FOUND LockEvent in plain text log!");
        }
    }
    
    println!("=== End of transaction {} ===", signature);
    Ok(())
}

