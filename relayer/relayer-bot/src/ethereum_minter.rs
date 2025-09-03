use web3::{
    api::Web3, contract::{Contract, Options}, transports::{Http, WebSocket},
    types::{Address, TransactionParameters, U256,H160,H256}
};
use hex::decode;
use std::error::Error;
use std::str::FromStr;
use std::env;
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256}; 
use web3::signing::SecretKeyRef;
use solana_sdk::{bs58, transaction};



pub fn string_to_ethereum_address(eth_address: &str) -> Result<H160, Box<dyn std::error::Error>> {
    // Remove "0x" prefix if present
    let cleaned_address = if eth_address.starts_with("0x") {
        &eth_address[2..]
    } else {
        eth_address
    };
    
    // Parse the hex string to H160
    let address = match H160::from_str(&format!("0x{}", cleaned_address)){
        Ok(address)=>{
            println!("[address in hex ]: {}", address);
            address
        },
        Err(e)=>{
            println!("[error in hex ]:{}",e);
            return Err(e.into());
        }
    };
    
    Ok(address)
}

pub fn solana_signature_to_bytes32(signature: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    // Decode base58 signature
    let sig_bytes = match  bs58::decode(signature).into_vec(){
        Ok(decode)=>{
            println!("[decoding signature worked]:{:?}", decode);
            decode
        }
        Err(e)=>{
            println!("[failed to decode the signature]:{}",e);
            return Err(e.into());
        }
    };
    
    // Hash it to get 32 bytes
    let hash = Keccak256::digest(&sig_bytes);
    let mut bytes32 = [0u8; 32];
    bytes32.copy_from_slice(&hash);
    
    Ok(bytes32)
}


//this function is for minting the fungible token
//1. get the  rpc,smart_contract address, and  the privatekey
//2. network transport for communication
//3 remove the private key prefix if 0x
pub async fn mint_wsol(to: &str, amount: u64, eth_address : &str, solana_tx_signature: &str) -> Result<(), Box<dyn Error>> {
    
    
    // 1. Connect to dev node
    // 1. Configuration from environment
    let eth_devnet_rpc_url = env::var("ETH_DEVNET_RPC_URL")
        .unwrap_or("https://eth-sepolia.g.alchemy.com".into());
    println!("the eth devnet {}", eth_devnet_rpc_url);
    let contract_addr = env::var("CWSOL_CONTRACT_ADDRESS")
        .expect("CWSOL_CONTRACT_ADDRESS must be set");
     println!("📋 Contract: {}", contract_addr);

    let private_key = env::var("ETH_DEVNET_PRIVATE_KEY")
        .expect("DEVNET_PRIVATE_KEY must be set");
      println!("🔑 Private key loaded: {}...{}", &private_key[..6], &private_key[private_key.len()-4..]);
    
     quick_contract_check(&contract_addr, &eth_devnet_rpc_url).await?;


    //transport
    let transport = match Http::new(&eth_devnet_rpc_url){
        Ok(protocol) => {
            println!("[protocol connection worked] : {:?}", protocol);
            protocol
        }
        Err(e)=>{
            println!("protocol connection error {}", e);
            return Ok(());
        }
    };
    let web3 = Web3::new(&transport);


    // 3. Parse private key (remove 0x prefix if present)
    let secp = Secp256k1::new();
    let private_key_clean = if private_key.starts_with("0x") {
        &private_key[2..]
    } else {
        &private_key
    };
    let private_key_bytes = match decode(private_key_clean){
        Ok(private) =>{
            //println!("private key in bytes: {:?}", private);
            private
        }
        Err(e)=>{
            println!("error in private key hex {}",e);
            return Ok(())
        }
    };
    let secret_key = match SecretKey::from_slice(&private_key_bytes){
        Ok(key)=>{
            println!("secretkeyin bytes {:?}", key);
            key
        }
        Err(e)=>{
            println!("error in getting the secretkey in bytes 32 {}",e);
            return Ok(());
        }
    };

    
    // 4. Derive Ethereum address
    let public_key = secret_key.public_key(&secp);
    let sender_address = H160::from_slice(
        &Keccak256::digest(&public_key.serialize_uncompressed()[1..65])[12..]
    );
    

    // 2. Get the accounts available on the node
    println!("Using account: {:?}", sender_address);
    
    // 3. Load contract ABI and address
    let artifact_json = include_str!("../../../smart_contracts/artifacts/contracts/CWSol/CWSol.sol/CWSol.json");
    let artifact: serde_json::Value = match serde_json::from_str(artifact_json){
        Ok(abi_in_json)=>{
            //println!("[abi is loaded correctly in json]: {}", abi_in_json);
            abi_in_json
        },
        Err(e)=> {
            println!("abi in json {} did not load well ", e);
            return Ok(())
        }
    };
    let abi = match artifact.get("abi"){
        Some(abi_value)=>{
           // println!("value of the abi is {}", abi_value);
            abi_value
        }
        None => {
            println!("value of abi not found");
            return Err("abi value not found ".into());
        }
    };
    
    let abi_bytes:Vec<u8> = match serde_json::to_vec(abi){
        Ok(vec_abi)=>{
            //println!("bytes  from json using vec {:?}", vec_abi);
            vec_abi
        }
        Err(e)=>{
            println!("error when turing the abi in json to vec");
            return Err(e.into());
        }
    };
   // println!("checking ABI {:?} bytes", abi_bytes);

    
    let contract_address: Address = match contract_addr.parse(){
        Ok(wsol_contract)=>{
            println!("Cwrapp sol address {} exist", wsol_contract);
            wsol_contract
        }
        Err(e)=>{
            println!("contract address not found ");
            return Err(e.into())
        }
    };

    let contract = match Contract::from_json(web3.eth(), contract_address, &abi_bytes){
        Ok(contract)=>{
            //println!("the full wsol contract {:?}",contract);
            contract
        }
        Err(e)=>{
            println!("error on the contract");
            return Err(e.into());
        }
    };

    //println!("Showing the Contract {:?}", contract);


    let etheruem_address = match string_to_ethereum_address(&eth_address){
        Ok(eth_address_type)=>{
            println!("getting the eth address {} ,from the string type", eth_address_type);
            eth_address_type
        }
        Err(e)=>{
            println!("failed to convert the eth address fro string format to eth address default format");
            return Err(e.into());
        }
    };


    // 4. Prepare transaction parameters
      let to_address: Address = etheruem_address;
    // Multiply lamports (1e9) by another 1e9 to get to 1e18
     let scaled_amount = U256::from(amount);
    //* U256::exp10(9); // 1e9
    //let amount_u256 = U256::from(amount);


    // 7. Get nonce for the sender
    //let nonce = web3.eth().transaction_count(sender_address, None).await?;
    let nonce = match web3.eth().transaction_count(sender_address, Some(web3::types::BlockNumber::Pending)).await{
        Ok(transaction_count)=>{
            println!("nonce was really created so a transaction has been counted ");
            transaction_count
        }
        Err(e) =>{
            println!("failed to count a transaction and nonce not created ");
            return Err(e.into())
        }
    };
   // println!("address {} and scaled Amount {}", to_address, scaled_amount);

   // let nonce = web3.eth().transaction_count(sender_address, None).await?;
    
    // 8. Get current gas price
    let gas_price = match web3.eth().gas_price().await{
        Ok(price)=>{
            println!("realtime gass {}", price);
            price
        }
        Err(e)=>{
            println!("failed to get real time gass price");
            return Err(e.into());
        }
    };



    // 5. Create transaction options
    let options = Options {
        gas: Some(300_000.into()),
        gas_price: Some(gas_price * 120 / 100),
        nonce: Some(nonce),
        ..Default::default()
    };

     // Convert signature to bytes32 for the contract
    let solana_tx_hash = match solana_signature_to_bytes32(solana_tx_signature){
        Ok(hash )=>{
            println!("hash solana transaction");
            hash
        }
        Err(e)=>{
            println!("signature was not hashed");
            return Err(e.into());
        }
    };
    let solana_tx_hash_h256 = H256::from_slice(&solana_tx_hash);


    // 10. Create SecretKeyRef for signing
    let secret_key_ref = SecretKeyRef::new(&secret_key);
    

    
    // 6. Call the mint function using the account from the node
    // We'll pass the from address directly to the call method
     let tx_hash = match contract
        .signed_call("mint", (to_address, scaled_amount, solana_tx_hash_h256), options,secret_key_ref)
        .await{
            Ok(call )=>{
                println!("calling the  mint  successful");
                call
            }

            Err(e) =>{
                println!("failed to call the mint ");
                return Err(e.into());
            }
        };

    println!("✅ Minting {} CWSOL to {}", amount, eth_address);
    
    println!("Mint transaction hash: {:?}", tx_hash);
    Ok(())
}


//checking the contract existance
pub async fn quick_contract_check(contract_addr: &str, rpc_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Quick contract check for: {}", contract_addr);
    
    let transport = web3::transports::Http::new(rpc_url)?;
    let web3 = web3::Web3::new(&transport);
    
    let contract_address: web3::types::H160 = contract_addr.parse()?;
    
    match web3.eth().code(contract_address, Some(web3::types::BlockNumber::Latest)).await {
        Ok(code) => {
            if code.0.is_empty() {
                println!("❌ NO CONTRACT found at {}", contract_address);
                println!("   Visit: https://sepolia.etherscan.io/address/{}", contract_address);
                return Err("Contract not deployed".into());
            } else {
                println!("✅ Contract exists! Code size: {} bytes", code.0.len());
            }
        }
        Err(e) => {
            println!("❌ Error checking contract: {}", e);
            return Err(e.into());
        }
    }
    Ok(())
}