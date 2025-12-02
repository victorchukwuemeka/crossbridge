use anchor_lang::prelude::*;
use crate::{records::{bridge_accounts::BridgeAccount, user_balance::UserBalance}, events::lock_event::LockEvent};
use sha2::{Digest, Sha256};
use orchard::{
    Address, Note, bundle::commitments, keys::{Diversifier, FullViewingKey, Scope, SpendingKey}, note::{ExtractedNoteCommitment, NoteCommitment, Nullifier, RandomSeed, Rho}, value::NoteValue
    
};
use orchard::keys::DiversifierIndex;
//use orchard::note::RandomSeed;
//use orchard::note::ExtractedNoteCommitment;
//use orchard::keys::Scope;
//use tap::pipe::Pipe;
    
//use orchard::keys::{SpendingKey, FullViewingKey, Scope};

#[derive(Accounts)]
pub struct LockSol<'info> {
    #[account(
        mut,
        seeds = [b"bridge_vault_v2"],
        bump = bridge_account.bump
    )]
    pub bridge_account: Account<'info, BridgeAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8 + 1 + 32 + 32, // discriminator + pubkey + u64 + u64 + bump+ 64 for privacy
        seeds = [b"user_balance_v3", user.key().as_ref()],
        bump
    )]
    pub user_balance: Account<'info, UserBalance>,


    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<LockSol>, amount: u64, eth_address: String, target_network:u8) -> Result<()> {
    const FEE_BPS:u64 = 1; //fee basis points in 0.01%
    let fee = amount * FEE_BPS/10000;
    let net_amount = amount.checked_sub(fee).ok_or(error!(ErrorCode::InsufficientAmountForFee))?;
    
   
    let transfer = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.user.key(),
        &ctx.accounts.bridge_account.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer,
        &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.bridge_account.to_account_info(),
        ]
    )?;
    

    //update bridge account 
    ctx.accounts.bridge_account.total_locked = ctx.accounts.bridge_account.total_locked
        .checked_add(net_amount).ok_or(error!(ErrorCode::Overflow))?;
    ctx.accounts.bridge_account.fees_collected = ctx.accounts.bridge_account.fees_collected
        .checked_add(fee).ok_or(error!(ErrorCode::Overflow))?;

    
   
    //so the commitement if for hash/proof for look event 
    //nullifier  for preventing double spend 
    let (commitment, nullifier) = generate_commitment(
        ctx.accounts.user.key(),
        amount,
        Clock::get()?.unix_timestamp,
    );



     //updating the user account 
    ctx.accounts.user_balance.user = ctx.accounts.user.key();
    ctx.accounts.user_balance.locked_amount = ctx.accounts.user_balance.locked_amount
    .checked_add(net_amount).ok_or(error!(ErrorCode::Overflow))?;
    ctx.accounts.user_balance.last_locked_amount = net_amount;
    ctx.accounts.user_balance.bump = ctx.bumps.user_balance;
    ctx.accounts.user_balance.commitment = commitment;
    ctx.accounts.user_balance.nullifier = nullifier;
    

    emit!(LockEvent {
        user: ctx.accounts.user.key(),
        eth_address,
        amount: net_amount,
        fee,
        target_network,
        timestamp: Clock::get()?.unix_timestamp,
        commitment,
        nullifier,
        privacy_request: true,
    });


    Ok(())
}


fn generate_commitment(
    user: Pubkey, 
    amount: u64, 
    timestamp: i64
) -> ([u8; 32], [u8; 32]) {
    
    let seed = derive_seed(user, timestamp);
    let sk = SpendingKey::from_bytes(seed).unwrap();
    let fvk = FullViewingKey::from(&sk);
    
    // Get recipient address
    let recipient: Address = fvk.address_at(0u64, Scope::External);
    let rho_bytes: [u8; 32] = rand::random();
    let rho = Rho::from_bytes(&rho_bytes).unwrap();
    let rseed = RandomSeed::from_bytes(seed, &rho).unwrap();

    
    // Create note
    let note = Note::from_parts(
        recipient,
        NoteValue::from_raw(amount),
        Rho::from_bytes(&[0u8; 32]).unwrap(),
        rseed,
        //rand::random(),
    ).unwrap();
    
    // Generate commitment
    let commitment = note.commitment();

    let extract_commit = ExtractedNoteCommitment::from(commitment);
    let commitment_bytes: [u8; 32] = extract_commit.to_bytes();

    // Generate nullifier
    let nullifier = note.nullifier(&fvk);
    let nullifier_bytes: [u8; 32] = nullifier.to_bytes();
    
    (commitment_bytes, nullifier_bytes)
}



fn derive_seed(user: Pubkey, timestamp: i64)->[u8; 32]{
    let mut hasher = Sha256::new();
    hasher.update(user.as_ref());
    hasher.update(timestamp.to_le_bytes());
    hasher.update(b"crosbridge-v1");
    hasher.finalize().into()
    
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient amount for fee")]
    InsufficientAmountForFee,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Invalid spending key")]
    InvalidSpendingKey,
}