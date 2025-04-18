use web3::transports::ws::WebSocket;
use web3::types::{Address, U256};
use web3::api::Web3;
use web3::ethabi::{Token, Uint};
use std::error::Error;

pub async fn mint_wsol(to: &str, amount: u64) -> Result<(), Box<dyn Error>> {
    println!("✅ Minting {} wSOL to {}", amount, to);

    //  local test private key
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; 

    // Connect to the local WebSocket provider (Ganache or Hardhat)
    let websocket = WebSocket::new("ws://127.0.0.1:8545").await?;
    let web3 = Web3::new(websocket);

    // Load your wallet from the private key
    let wallet = web3::signing::Key::from_private_key(private_key)?;

    // Load contract ABI 
   // let abi = include_str!("../abi/Wsol.json");
    let abi = include_str!("../../../smart_contracts/artifacts/contracts/wSol/WSol.sol/WSol.json");
    let contract_address: Address = "0xYourContractAddress".parse()?;
    
    // Parse the ABI and prepare the contract
    let contract = web3.eth().contract(web3::ethabi::Contract::load(abi.as_bytes())?);
    let to_address: Address = to.parse()?;

    // Set up the parameters for the mint function
    let mint_function = contract.method("mint", (to_address, U256::from(amount)))?;

    // Send the transaction
    let tx_hash = mint_function.send().await?;

    println!("Mint transaction sent with hash: {:?}", tx_hash);

    Ok(())
}
