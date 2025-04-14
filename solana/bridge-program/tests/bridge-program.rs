use anchor_lang::{prelude::*};
use anchor_lang::solana_program::system_program;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use anchor_client::solana_sdk::pubkey::Pubkey;

#[tokio::test]
async fn test_initialize_and_lock_sol() {
    let program_id = Pubkey::from_str("28AQpwDXyQPTkcuJweUQFfAMqTkDZfNME71Anic7o5rM").unwrap();
    let client = anchor_client::Client::new_with_options(
        solana_client::rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string()),
        Keypair::new(),
        anchor_client::ClientOptions::default(),
    );

    let program = client.program(program_id);

    // Step 1: Initialize the bridge account
    let user = Keypair::new();
    let bridge_account = Keypair::new();

    let ix_initialize = program
        .request()
        .accounts(bridge_program::accounts::Initialize {
            bridge_account: bridge_account.pubkey(),
            user: user.pubkey(),
            system_program: system_program::id(),
        })
        .signers(&[&user, &bridge_account])
        .instruction();
    
    let mut tx = Transaction::new_with_payer(&[ix_initialize], Some(&user.pubkey()));
    client.rpc().send_and_confirm_transaction(&tx).unwrap();

    // Verify the bridge account is initialized (optional: check state on-chain)

    // Step 2: Lock some SOL
    let amount = 1000000000; // 1 SOL in lamports
    let ix_lock_sol = program
        .request()
        .accounts(bridge_program::accounts::LockSol {
            bridge_account: bridge_account.pubkey(),
            user: user.pubkey(),
            system_program: system_program::id(),
        })
        .args(bridge_program::instruction::LockSol { amount })
        .signers(&[&user])
        .instruction();
    
    let mut tx = Transaction::new_with_payer(&[ix_lock_sol], Some(&user.pubkey()));
    client.rpc().send_and_confirm_transaction(&tx).unwrap();

    // Optional: Assert that the total_locked in the bridge account has been updated
}
