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
use solana_sdk::bs58;


// 1: Simple hash-based derivation
/*pub fn solana_to_ethereum_address(solana_address: &str) -> Result<Address, Box<dyn std::error::Error>> {
    // Decode the Solana address from base58
    let solana_bytes = bs58::decode(solana_address).into_vec()?;
    
    // Hash the Solana address bytes to get Ethereum address
    let hash = Keccak256::digest(&solana_bytes);
    
    // Take the last 20 bytes as Ethereum address
    let eth_address = H160::from_slice(&hash[12..]);
    
    Ok(eth_address)
}*/

pub fn string_to_ethereum_address(eth_address: &str) -> Result<H160, Box<dyn std::error::Error>> {
    // Remove "0x" prefix if present
    let cleaned_address = if eth_address.starts_with("0x") {
        &eth_address[2..]
    } else {
        eth_address
    };
    
    // Parse the hex string to H160
    let address = H160::from_str(&format!("0x{}", cleaned_address))?;
    
    Ok(address)
}

pub fn solana_signature_to_bytes32(signature: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    // Decode base58 signature
    let sig_bytes = bs58::decode(signature).into_vec()?;
    
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
    println!("✅ Minting {} wSOL to {}", amount, to);
    
    // 1. Connect to dev node
    // 1. Configuration from environment
    let eth_devnet_rpc_url = env::var("ETH_DEVNET_RPC_URL")
        .unwrap_or("https://eth-sepolia.g.alchemy.com".into());
    let contract_addr = env::var("WSOL_CONTRACT_ADDRESS")
        .expect("WSOL_CONTRACT_ADDRESS must be set");
    let private_key = env::var("DEVNET_PRIVATE_KEY")
        .expect("DEVNET_PRIVATE_KEY must be set");
    
    //transport
    let transport = Http::new(&eth_devnet_rpc_url)?;
    let web3 = Web3::new(&transport);


    // 3. Parse private key (remove 0x prefix if present)
    let secp = Secp256k1::new();
    let private_key_clean = if private_key.starts_with("0x") {
        &private_key[2..]
    } else {
        &private_key
    };
    let private_key_bytes = decode(private_key_clean)?;
    let secret_key = SecretKey::from_slice(&private_key_bytes)?;

    
    // 4. Derive Ethereum address
    let public_key = secret_key.public_key(&secp);
    let sender_address = H160::from_slice(
        &Keccak256::digest(&public_key.serialize_uncompressed()[1..65])[12..]
    );
    

    // 2. Get the accounts available on the node
    //println!("Using account: {:?}", sender_address);
    
    // 3. Load contract ABI and address
    let artifact_json = include_str!("../../../smart_contracts/artifacts/contracts/wSol/WSol.sol/WSol.json");
    let artifact: serde_json::Value = serde_json::from_str(artifact_json)?;
    let abi = artifact.get("abi").ok_or("ABI not found in artifact")?;
    let abi_bytes = serde_json::to_vec(abi)?;
   // println!("checking ABI {:?} bytes", abi_bytes);

    
    let contract_address: Address = contract_addr.parse()?;
    let contract = Contract::from_json(web3.eth(), contract_address, &abi_bytes)?;

    //println!("Showing the Contract {:?}", contract);


    let etheruem_address = string_to_ethereum_address(&eth_address)?;


    // 4. Prepare transaction parameters
    let to_address: Address = etheruem_address;
    let amount_u256 = U256::from(amount);


    // 7. Get nonce for the sender
    //let nonce = web3.eth().transaction_count(sender_address, None).await?;
    let nonce = web3.eth().transaction_count(sender_address, Some(web3::types::BlockNumber::Pending)).await?;
    
      // 8. Get current gas price
    let gas_price = web3.eth().gas_price().await?;


    // 5. Create transaction options
    let options = Options {
        gas: Some(300_000.into()),
        gas_price: Some(gas_price * 120 / 100),
        nonce: Some(nonce),
        ..Default::default()
    };

     // Convert signature to bytes32 for the contract
    let solana_tx_hash = solana_signature_to_bytes32(solana_tx_signature)?;
    let solana_tx_hash_h256 = H256::from_slice(&solana_tx_hash);


    // 10. Create SecretKeyRef for signing
    let secret_key_ref = SecretKeyRef::new(&secret_key);
    

    
    // 6. Call the mint function using the account from the node
    // We'll pass the from address directly to the call method
    let tx_hash = contract
        .signed_call("mint", (to_address, amount_u256, solana_tx_hash_h256), options,secret_key_ref)
       .await?;
    
    println!("Mint transaction hash: {:?}", tx_hash);
    Ok(())
}