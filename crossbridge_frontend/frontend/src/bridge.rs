use leptos::*;
use web_sys::console;

use leptos::prelude::{Set, ElementChild, OnAttribute, ClassAttribute, Get};
//use leptos::prelude::signal;
use crate::event_target_value;
//use crate::Show;
use solana_program::{
    pubkey::Pubkey,
    system_instruction,
    instruction::Instruction
};
use std::str::FromStr;

use solana_sdk::{
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use crate::models::WalletState;
use leptos::context::use_context;
use wasm_bindgen::JsCast;
use web_sys::window;
use log::{error, warn};
use js_sys::{Object,Reflect};
use wasm_bindgen::JsValue;
use leptos::prelude::PropAttribute;
//use solana_program::serde_varint::serialize;
//use base64::{engine::general_purpose, Engine as _};
use bincode;
use solana_sdk::transaction::VersionedTransaction;


#[component]
pub fn BridgeActions() -> impl IntoView {
    
    //getting the current state of the wallet .
    let WalletState {
        sol_address,
        amount,
        is_connected,
        ..
    } = use_context::<WalletState>()
        .expect("WalletState should be provided");


    
    //locking the sol to the solana network in the devnet
    let lock_sol = move |_|{

        //get the public key from the wallet .
        let wallet_pubkey = match Pubkey::from_str(&sol_address.get()){
            Ok(pk) => pk,
            Err(_) => {
                log::error!("Invalid wallet address");
                return;
            }
        };

        //generate the vault pda 
        let program_id = Pubkey::from_str("911VdUg43JGvomS2eCqKHJcUZ6J9SCjb371w6Xst7YMD").unwrap();
        let (vault_pda, _bump) = Pubkey::find_program_address(
            &[b"sol_vault", wallet_pubkey.as_ref()],
            &program_id
        );

        //convert sol to lamport 
        let amount_lamports = (amount.get() * 1_000_000_000.0) as u64;



       
        //create transfer instruction
        let ix = system_instruction::transfer(
            &wallet_pubkey,
            &vault_pda,
            amount_lamports,
        );

        // Create transaction
        let mut tx = Transaction::new_with_payer(&[ix], Some(&wallet_pubkey));


        //getting the last blockhash 
        let rpc_url  ="https://api.devnet.solana.com".to_string();
        let client = RpcClient::new(rpc_url);

        let recent_block = match client.get_latest_blockhash(){
             Ok(hash) => hash,
             Err(e) => {
                log::error!("Failed to get recent blockhash: {}", e);
                return;
            }
        };

        tx.message.recent_blockhash = recent_block;
        


        // Get window and solana object
        let window = match web_sys::window() {
            Some(w) => w,
            None => {
                log::error!("Window object not available");
                return;
            }
        };

        //get the solana Js Object injected by phantom wallet 
        let solana = match window.get("solana"){
            Some(s) => s,
            None => {
                log::error!("Solana Object not available");
                return;
            }
        };
        
         // 7. Convert to VersionedTransaction (CRITICAL CHANGE)
         let versioned_tx = VersionedTransaction::from(tx);

        //serialize the transfer so phantom can use it 
        let serialized_tx = bincode::serialize(&versioned_tx).unwrap();
        let tx_js = js_sys::Uint8Array::from(&serialized_tx[..]).into();
       // let base64_tx = general_purpose::STANDARD.encode(&serialized_tx);
       // let tx_js = JsValue::from_str(&base64_tx);

        
        // Get the signAndSendTransaction function
        let sign_send_fn = unsafe {
            js_sys::Reflect::get(&solana, &JsValue::from_str("signAndSendTransaction"))
        }.expect("signAndSendTransaction not found");
        let sign_send_fn: js_sys::Function = sign_send_fn
         .dyn_into()
        .expect("solana.signAndSendTransaction must be a function");

        

        // 10. Handle promise (async operation)
        let promise = sign_send_fn.call1(&solana, &tx_js)
        .expect("Failed to call signAndSendTransaction")
        .dyn_into::<js_sys::Promise>()
        .expect("Function should return a promise");
        
        let future = wasm_bindgen_futures::JsFuture::from(promise);
        wasm_bindgen_futures::spawn_local(async move {
            match future.await {
                Ok(_) => log::info!("Transaction successful!"),
                Err(e) => log::error!("Transaction failed: {:?}", e),
            }
        });
       
    };

    view! {
        <div class="action-panel">
             <input 
                    type="number" 
                    step="0.1"
                    on:input=move |ev| amount.set(event_target_value(&ev).parse().unwrap_or(0.0))
                    prop:value=amount
                    placeholder="SOL amount"
                />
                <button on:click=lock_sol>
                    "Lock SOL "
                </button>

        </div>
        <br/>
    }
}