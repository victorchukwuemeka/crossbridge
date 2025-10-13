use anchor_lang::solana_program::pubkey;
use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::error::Error;


#[derive(Debug)]
pub struct UserLockState{
    pub user : Pubkey,
    pub amount : u64,
    pub locked_at : i64,
} 


pub struct SolanaStateClient{
    rpc_client: RpcClient,
    program_id : Pubkey,
}



impl SolanaStateClient {

    pub fn new(rpc_url: String , program_id: Pubkey)->Self{
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url, 
            CommitmentConfig::confirmed()
        );
        Self { rpc_client, program_id}
    }

    


}