use web3::{
    api::Web3, contract::{Contract, Options}, transports::{Http, WebSocket},
    types::{Address, TransactionParameters, U256,H160}
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
pub fn solana_to_ethereum_address(solana_address: &str) -> Result<Address, Box<dyn std::error::Error>> {
    // Decode the Solana address from base58
    let solana_bytes = bs58::decode(solana_address).into_vec()?;
    
    // Hash the Solana address bytes to get Ethereum address
    let hash = Keccak256::digest(&solana_bytes);
    
    // Take the last 20 bytes as Ethereum address
    let eth_address = H160::from_slice(&hash[12..]);
    
    Ok(eth_address)
}

pub async fn mint_wsol(to: &str, amount: u64) -> Result<(), Box<dyn Error>> {
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


    let etheruem_address = solana_to_ethereum_address(to)?;


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




    // 10. Create SecretKeyRef for signing
    let secret_key_ref = SecretKeyRef::new(&secret_key);
    
    
    // 6. Call the mint function using the account from the node
    // We'll pass the from address directly to the call method
    let tx_hash = contract
        .signed_call("mint", (to_address, amount_u256), options,secret_key_ref)
       .await?;
    
    println!("Mint transaction hash: {:?}", tx_hash);
    Ok(())
}