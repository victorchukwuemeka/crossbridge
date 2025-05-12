use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use log::{error, warn};
use js_sys::{Object,Reflect};
use wasm_bindgen::JsValue;  
use leptos::prelude::signal;
use leptos::prelude::{Set, ElementChild, OnAttribute, ClassAttribute, Get};
use std::str::FromStr;
use solana_program::{
    pubkey::Pubkey,
    system_instruction,
    instruction::Instruction
};
use leptos::prelude::PropAttribute;

use solana_sdk::{
    transaction::Transaction,
    //pubkey::Pubkey,
    //wasm_bindgen::prelude::*,
};
use leptos::prelude::event_target_value;


//use solana_client_wasm::solana_sdk::transaction::Transaction;


#[wasm_bindgen]
//the extern "C" is for talking to a language outside rust
//is the standard protocol for it .
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = solana)]
    static SOLANA: JsValue;
    #[wasm_bindgen(js_namespace = window, js_name = ethereum)]
    static ETHEREUM: JsValue;
}

#[component]
pub fn WalletStatus() -> impl IntoView {
    let (sol_address, set_sol) = signal("Not connected".to_string());
    let (eth_address, set_eth) = signal("Not connected".to_string());
    let (amount, set_amount) = signal(0.0);
    let (is_connected, set_is_connected) = signal(false);



    let rpc_url = "https://api.devnet.solana.com";
    //let wallet_pubkey = Pubkey::from_str(&sol_address.get())?;


    let connect_solana = move |_| {



        //get the browser window object
        let window = match  web_sys::window(){
            Some(w) => w,
            None => {
                log::error!("Window object not available");
                return;
            }
        };
        
        // Get solana - FIXED:
        //remember the wallet inject the solana object.
        let solana = match window.get("solana") {
            Some(s) => s,  
            None => {
                set_sol.set("Phantom Wallet not detected".into());
                return;
            }
        };


        //get the connect
        let connect_fn = match unsafe { js_sys::Reflect::get(&solana, &JsValue::from_str("connect")) } {
            Ok(f) => f,
            Err(_) => {
                set_sol.set("Connect method not available".into());
                return;
            }
        };
    

        //convert to callable function
        let connect_fn = match connect_fn.dyn_into::<js_sys::Function>(){
            Ok(f) => f,
            Err(_) =>{
                set_sol.set("Invalid Connect function".into());
                return;
            }
        };

         // 5. Execute connection
         if let Err(e) = connect_fn.call0(&solana) {
            log::error!("Connection failed: {:?}", e);
            set_sol.set("Connection rejected".into());
            return;
        }

        //get the wallet public key 
        let pubkey = match  unsafe{Reflect::get(&solana,&JsValue::from_str("publicKey"))}{
            Ok(pk) => pk,
            Err(_) => {
                set_sol.set("Failed to get public key".into());
                return;
            }
        };

        //convert it to make sure it is in string 
        let pubkey_str = match pubkey.as_string(){
            Some(s) => s,
            None => {"Failed to parse public key".to_string()}
        };



        set_sol.set(pubkey_str); // Replace with real connection
        set_is_connected.set(true);
    };


    let lock_sol = move |_|{
        let wallet_pubkey = match Pubkey::from_str(&sol_address.get()){
            Ok(pk) => pk,
            Err(_) => {
                log::error!("Invalid wallet address");
                return;
            }
        };

        //generate the vault pda 
        let program_id = Pubkey::from_str("llllll").unwrap();
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
        let tx = Transaction::new_with_payer(&[ix], Some(&wallet_pubkey));

        // Get window and solana object
        let window = match web_sys::window() {
            Some(w) => w,
            None => {
                log::error!("Window object not available");
                return;
            }
        };


        let solana = match window.get("solana"){
            Some(s) => s,
            None => {
                log::error!("Solana Object not available");
                return;
            }
        };

        match solana.unchecked_ref::<js_sys::Function>()
            .call2(
                &JsValue::NULL,
                &JsValue::from_str("signAndSendTransaction"),
                &JsValue::from(tx),
            ) {
            Ok(_) => log::info!("SOL locked successfully!"),
            Err(e) => log::error!("Lock failed: {:?}", e),
        } 
       

    };



    let connect_ethereum = move |_| {
       // let window = web_sys::window().unwrap();
        let window = match web_sys::window(){
            Some(w) => w,
            None => {
                log::error!("windows object not found");
                return;
            }
        };

        let ethereum = match window.get("ethereum"){
            Some(e) => e,
            None => {
                set_eth.set("Metamask Not found" .into());
                return;
            }
        };



        let connect_eth_fn =  match unsafe{js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))} {
            Ok(e) => e,
            Err(_) => {
                set_eth.set("Connection method not available".into());
                return;
            }
        };

        let connect_eth_fn =  match connect_eth_fn.dyn_into::<js_sys::Function>() {
            Ok(e) => e,
            Err(_) => {
                set_eth.set("Invalid Connection method".into());
                return;
            }
        };

        // 5. Execute connection
        if let Err(e) = connect_eth_fn.call1(&ethereum,&JsValue::from_str("eth_requestAccounts")) {
            log::error!("Connection failed: {:?}", e);
            set_sol.set("Connection rejected".into());
            return;
        }


        set_eth.set("0x892..4f1".into()); // Replace with real connection
    };

    view! {
        <div class="wallet-status">
            <p>"Network: Solana Devnet | Ethereum Sepolia"</p>
            <div>
                <button on:click=connect_solana>
                    {move || format!("SOL: {}", sol_address.get())}
                </button>
                <button on:click=connect_ethereum>
                    {move || format!("ETH: {}", eth_address.get())}
                </button>
            </div>
            <div class="lock-section" class:hidden=move || !is_connected.get()>
                <input 
                    type="number" 
                    step="0.1"
                    on:input=move |ev| set_amount.set(event_target_value(&ev).parse().unwrap_or(0.0))
                    prop:value=amount
                    placeholder="SOL amount"
                />
                <button on:click=lock_sol>
                    "Lock SOL"
                </button>
            </div>
        </div>
    }
}
