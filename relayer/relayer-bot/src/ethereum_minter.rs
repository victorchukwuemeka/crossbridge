use web3::{
    transports::WebSocket,
    types::{Address, U256, TransactionParameters},
    contract::{Contract, Options},
    api::Web3,
};
use hex::decode;
use std::error::Error;
use std::str::FromStr;

pub async fn mint_wsol(to: &str, amount: u64) -> Result<(), Box<dyn Error>> {
    println!("✅ Minting {} wSOL to {}", amount, to);

    // 1. Connect to local node (Ganache/Hardhat)
    let websocket = WebSocket::new("ws://127.0.0.1:8545").await?;
    let web3 = Web3::new(websocket);

    // 2. Get the accounts available on the node
    let accounts = web3.eth().accounts().await?;
    if accounts.is_empty() {
        return Err("No accounts found on the node".into());
    }
    
    let from_account = accounts[0]; // Use the first account
    println!("Using account: {:?}", from_account);

    // 3. Load contract ABI and address
    let abi = include_str!("../../../smart_contracts/artifacts/contracts/wSol/WSol.sol/WSol.json");
    let contract_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse()?;
    let contract = Contract::from_json(web3.eth(), contract_address, abi.as_bytes())?;

    // 4. Prepare transaction parameters
    let to_address: Address = to.parse()?;
    let amount_u256 = U256::from(amount);

    // 5. Create transaction options
    let options = Options {
        gas: Some(300_000.into()),
        ..Default::default()
    };
    
    // 6. Call the mint function using the account from the node
    // We'll pass the from address directly to the call method
    let tx_hash = contract
        .call("mint", (to_address, amount_u256), from_account, options)
        .await?;

    println!("Mint transaction hash: {:?}", tx_hash);
    Ok(())
}