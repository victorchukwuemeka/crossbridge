use anchor_lang::{prelude::*};
use anchor_lang::solana_program::system_program;
use anchor_client::{
    Client, Cluster,
    solana_sdk::{
        signature::{Keypair, Signer},
        pubkey::Pubkey,
        native_token::LAMPORTS_PER_SOL,
    },
};
use std::rc::Rc;
use std::str::FromStr;

#[tokio::test]
async fn test_initialize_lock_unlock() -> anyhow::Result<()> {
    // Setup
    let program_id = Pubkey::from_str("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM")?;
    let user = Keypair::new();
    let bridge_account = Keypair::new();

    // Connect to devnet
    let client = Client::new_with_options(
        Cluster::Devnet,
        Rc::new(user.clone()),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id);

    // Airdrop 2 SOL to user
    println!("Airdropping 2 SOL to user: {}", user.pubkey());
    let sig = program
        .rpc()
        .request_airdrop(&user.pubkey(), 2 * LAMPORTS_PER_SOL)?;
    program.rpc().confirm_transaction(&sig)?;

    // Initialize
    println!("Initializing bridge account...");
    program
        .request()
        .accounts(bridge_program::accounts::Initialize {
            bridge_account: bridge_account.pubkey(),
            user: user.pubkey(),
            system_program: system_program::ID,
        })
        .args(bridge_program::instruction::Initialize {})
        .signer(&bridge_account)
        .send()
        .await?;

    // Lock SOL
    let amount = 1 * LAMPORTS_PER_SOL;
    println!("Locking 1 SOL...");
    program
        .request()
        .accounts(bridge_program::accounts::LockSol {
            bridge_account: bridge_account.pubkey(),
            user: user.pubkey(),
            system_program: system_program::ID,
        })
        .args(bridge_program::instruction::LockSol { amount })
        .send()
        .await?;

    // Unlock SOL
    println!("Unlocking 1 SOL...");
    program
        .request()
        .accounts(bridge_program::accounts::UnLockSol {
            bridge_account: bridge_account.pubkey(),
            user: user.pubkey(),
            system_program: system_program::ID,
        })
        .args(bridge_program::instruction::UnLockSol { amount })
        .send()
        .await?;

    // (Optional) Fetch and print bridge account state
    let bridge_state: bridge_program::BridgeAccount = program.account(bridge_account.pubkey()).await?;

    println!("Bridge total locked: {} SOL", bridge_state.total_locked as f64 / LAMPORTS_PER_SOL as f64);

    Ok(())
}
