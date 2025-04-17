use std::error::Error;


pub async fn mint_wsol(to: &str, amount: u64) -> Result<(), Box<dyn Error>> {
    println!("✅ Minting {} wSOL to {}", amount, to);

    // Load your signer/wallet
    let wallet = Wallet::from(private_key);
    let provider = Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/...").await?;
    let client = SignerMiddleware::new(provider, wallet);

    // Load contract
    let abi = include_str!("../abi/Wsol.json");
    let address = "0xYourContractAddress".parse::<Address>()?;
    let contract = Contract::new(address, abi.parse()?, client);

    // Call mint()
    contract.method::<_, H256>("mint", (to, amount))?.send().await?;

    Ok(())
}
