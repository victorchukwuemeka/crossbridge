use solana_client::send_and_confirm_transactions_in_parallel;
//use bs58::decode;
use web3::{
    api::Web3, contract::{Contract, Options}, transports::{Http, WebSocket},
    types::{Address, TransactionParameters, U256,H160,H256}
};


use std::error::Error;
use std::env;
use futures::future::ok;
use hex::decode; 
use secp256k1::{Secp256k1, SecretKey};
use sha3::{Digest, Keccak256}; 
use crate::utils::string_to_ethereum_address;
use crate::utils::solana_signature_to_bytes32;
use web3::signing::SecretKeyRef;


/**
 * the mint function for minting the base_cwsol 
*/
pub async fn mint_base_cwsol(to: &str, amount: u64, base_address: &str, solana_tx_signature: &str )
->Result<(), Box<dyn Error>>
{   


    println!("[START MINTING CWSOL ON BASE ]!");
    /**
     * configuartion of the rpc, addr  and the private key
     * all from the env .
    */
     let base_testnet_rpc_url = env::var("BASE_TESTNET_RPC")
     .unwrap_or("https://sepolia.base.org".into());
     let base_contract_addr = env::var("BASE_CWSOL_CONTRACT_ADDRESS")
      .expect("CWsol Contract Address for Base Must be Set");
     let base_private_key = env::var("BASE_TESTNET_PRIVATE_KEY")
      .expect("the private key is needed");

    /**
     * transport for protocol connection that helps us communicate with the 
     * rpc
     */
    let transport = match Http::new(&base_testnet_rpc_url) {
        Ok(protocol)=> protocol,
        Err(_)=> return Ok(()),
    };
    let web3 = Web3::new(transport);

    /**
     * for the private key we 
     * 1. we remove the 0x because is not needed for our convertion of hex to bytes 
     * 2. after removing the 0x we then convert the key to bytes 
     * 3. from btyes we turn then to secret objects that can sign a transaction
     * 4. we do all this cause that the only way the library we are working with
     * 5. can actually understand the data just like our computer always compile 
     * 6. char from human readable to machine readable
     */

    //private key without the 0x
    let striped_base_private_key = base_private_key.strip_prefix("0x").unwrap_or(&base_private_key);
    
    //private key in bytes 
    let striped_base_private_key_bytes = match decode(striped_base_private_key){
        Ok(b)=> b,
        Err(_e)=> return Ok(())
    };

    //convert the bytes to secret key
    let base_secret_key = match SecretKey::from_slice(&striped_base_private_key_bytes){
        Ok(bsk)=>bsk,
        Err(_e)=> return Ok(())
    };
    
    //derive the address 
    let secp = Secp256k1::new();
    let base_public_key = base_secret_key.public_key(&secp);
    let sender_base_address = H160::from_slice(
        &Keccak256::digest(&base_public_key.serialize_uncompressed()[1..65])[12..]
    );

    //load abi of the base contract
    let artifact_json_base = include_str!("../base_abi/CWSol.json");
    let artifact_base:serde_json::Value = match serde_json::from_str(artifact_json_base){
        Ok(base_abi_in_json) => base_abi_in_json,
        Err(_e) => return Ok(())
    };
    let base_abi = match artifact_base.get("abi"){
        Some(abi) => abi,
        None => return Ok(())
    };

    let base_abi_bytes = match serde_json::to_vec(base_abi){
        Ok(abi_bytes) => abi_bytes,
        Err(_e) => return Ok(())
    };

    let contract_addr:Address = match base_contract_addr.parse(){
        Ok(base_contract)=>{
            println!("Cwrapp sol address {} exist", base_contract);
            base_contract
        }
        Err(e)=>{
            println!("contract address not found ");
            return Err(e.into())
        }
    };

    let base_contract = match Contract::from_json(web3.eth(),contract_addr, &base_abi_bytes){
        Ok(con) => {
            println!("[BASE CONTRACT ABI]");
            con
        },
        Err(_e)=> return Ok(())
    };

    let base_addr = match string_to_ethereum_address(base_address){
        Ok(addr) => {
            println!("✅ Successfully parsed base address: {:?}", addr);
            addr
        }
        Err(_e) =>  {
            println!("❌ Error parsing base address '{}': {:?}", base_address, _e);
            return Err(_e.into())
        }
    };

    // 4. Prepare transaction parameters
    let to_address: Address = base_addr;
    // Multiply lamports (1e9) by another 1e9 to get to 1e18
    let scaled_amount = U256::from(amount);
    

    //get the nonce to avoid double transaction
    println!("🔍 Getting nonce for address: {:?}", sender_base_address);
    let nonce_base = match web3.eth().transaction_count(
        sender_base_address, Some(web3::types::BlockNumber::Pending)
    ).await{
        Ok(count)=>{
            println!("✅ Nonce retrieved: {}", count);
            count
        }
        Err(_e)=>{
            println!("❌ Failed to get nonce: {:?}", _e);
            return Ok(())
        }
        
    };

    //get current gas fee 
    let base_gas_price = match web3.eth().gas_price().await{
        Ok(price) => {
            println!("✅ Gas price: {}", price);
            price
        }
        Err(_e) =>{
             println!("❌ Failed to get gas price: {:?}", _e);
             return Ok(())
        }
    };

    /**
     * in evm transactions 
     * We need the Options to tell the blockchain how much gas we allow,
     *  what fee we’ll pay, and which nonce to use, 
     * so the transaction can be executed correctly and in order.
     * Without setting these, the transaction could fail (out of gas),
     *  get delayed (low fee), or be rejected (wrong nonce).
     */
    
    // 5. Create transaction options
    let base_options = Options {
        gas: Some(300_000.into()),
        gas_price: Some(base_gas_price * 120 / 100),
        nonce: Some(nonce_base),
        ..Default::default()
    };

    /**
     * convert the solana tx signature to bytes then hash it 
    */
    println!("🔍 Converting Solana signature: {}", solana_tx_signature);
    let solana_tx_bytes = match solana_signature_to_bytes32(solana_tx_signature){
        Ok(hash) => {
            println!("✅ Solana signature converted");
            hash
        }
        Err(_e)=> {
            println!("❌ Failed to convert Solana signature: {:?}", _e);
            return Ok(())
        }
    };
    //then convert the hash to H256 used my evms
    let solana_tx_hash_H256 = H256::from_slice(&solana_tx_bytes);

    //secret key ref for signing the transaction
    let secret_key_ref = SecretKeyRef::new(&base_secret_key);
    println!("[SECRET KEY REF ]!");



    let base_tx_hash_signed = match base_contract.signed_call(
        "mint", (to_address,scaled_amount,solana_tx_hash_H256), 
        base_options, secret_key_ref
    ).await{
        Ok(signed) => {
            println!("\x1b[1;31m===========Signed============\x1b[0m");
            signed
        },
        Err(_e) => {
            println!("\x1b[1;31m===========FAILED============\x1b[0m");
            return Err(_e.into())
        }
        
    };
    
    println!("CROSSBRIDGE WSOL HAS BEEN MINTED ON BASE!");
    println!("\x1b[1;31m==============================\x1b[0m");
    println!("\x1b[1;33mCROSSBRIDGE WSOL HAS BEEN MINTED ON BASE!\x1b[0m");
    println!("\x1b[1;31m==============================\x1b[0m");

    println!("✅ Minting {} CWSOL to   base address {}", amount, base_address);
    
    println!("Mint transaction hash: {:?}", base_tx_hash_signed);
    
    Ok(())
}