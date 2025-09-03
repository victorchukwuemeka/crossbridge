use web3::types::H160;
use std::str::FromStr;
use sha3::{Digest, Keccak256}; 

pub fn string_to_ethereum_address(eth_address: &str) -> Result<H160, Box<dyn std::error::Error>> {
    // Remove "0x" prefix if present
    let cleaned_address = if eth_address.starts_with("0x") {
        &eth_address[2..]
    } else {
        eth_address
    };
    
    // Parse the hex string to H160
    let address = match H160::from_str(&format!("0x{}", cleaned_address)) {
        Ok(address) => {
            println!("[address in hex ]: {}", address);
            address
        },
        Err(e) => {
            println!("[error in hex ]: {}", e);
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
