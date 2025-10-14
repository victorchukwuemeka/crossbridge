use anchor_lang::solana_program::pubkey;
//use secp256k1::ecdsa::Signature;
use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::error::Error;
use solana_sdk::signature::Signature;
use std::str::FromStr;
use solana_transaction_status::UiTransactionEncoding;




#[derive(Debug)]
pub struct UserLockState{
    pub user : Pubkey,
    pub amount : u64,
    pub bump : u8,
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
    
    //getting the user pda cause we want to use it for transaction verification
    pub fn get_user_balance_pda(&self, user:Pubkey)->Pubkey{
        let (pda, _bump) = Pubkey::find_program_address(
            &[b"user_balance", user.as_ref()],
            &self.program_id
        );
        pda
    }
    
    //getting the bridge pda also used for verification
    pub fn get_bridge_account_pda(&self)->Pubkey{
        let (pda, _bump) = Pubkey::find_program_address(
            &[b"bridge_vault_v2"], 
            &self.program_id
        );
        pda 
    }

    //check transaction finality 
    pub async fn is_transaction_finalized(&self, signature:&str)->Result<bool,Box<dyn Error>> {
        let sig=  Signature::from_str(signature)?;
        match self.rpc_client.get_transaction(&sig,  UiTransactionEncoding::Json){
            Ok(_)=> Ok(true),
            Err(_) => Ok(false),
        }
        
    }

    //get user lock state from pda 
    pub fn get_user_lock_state(&self, user: Pubkey)->Result<Option<UserLockState>, Box<dyn Error>>{
        let user_balance_pda  = self.get_user_balance_pda(user);

        println!(" Checking PDA: {} for user: {}", user_balance_pda, user);

        match self.rpc_client.get_account_data(&user_balance_pda){
            Ok(account_data) => {
                 println!("✅ PDA account exists! Deserializing...");
                
                // Deserialize UserBalance account (adjust based on your actual struct)
                let user_balance = self.deserialize_user_balance(&account_data)?;

                Ok(Some(UserLockState {
                    user: user_balance.user,
                    amount: user_balance.locked_amount,
                    bump : user_balance.bump
                }))
            },
            Err(_) => {
                println!("❌ PDA account not found");
                Ok(None)
            }
        }

    }


    fn deserialize_user_balance(&self, data: &[u8])->Result<UserBalance, Box<dyn Error>>{
        let data  = &data[8..];// remember the descriminator i' skipping

        // Parse based on your UserBalance struct:
        //space = 8 + 32 + 8 + 1, // discriminator + pubkey + u64 + bump
        let user_bytes: [u8; 32] = data[0..32].try_into()?;
        let user = Pubkey::new_from_array(user_bytes);

        let amount_bytes: [u8; 8] = data[32..40].try_into()?;
        let locked_amount = u64::from_le_bytes(amount_bytes);

        let bump = data[40];
        
        Ok(UserBalance {
            user,
            locked_amount,
            bump,
        })
    }


}


#[derive(Debug)]
struct UserBalance {
    pub user: Pubkey,
    pub locked_amount: u64,
    pub bump: u8,
}



