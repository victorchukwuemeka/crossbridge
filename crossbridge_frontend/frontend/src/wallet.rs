use leptos::*;
use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;  
use leptos::prelude::signal;
use leptos::prelude::{Set, ElementChild, OnAttribute, ClassAttribute, Get};

use solana_program::{
    pubkey::Pubkey,
    system_instruction,
    instruction::Instruction
};
use leptos::prelude::PropAttribute;
use leptos::prelude::event_target_value;
use leptos::context::use_context;
use crate::models::WalletState;

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
//    let (sol_address, set_sol) = signal("Not connected".to_string());
//    let (eth_address, set_eth) = signal("Not connected".to_string());
    
    //let (amount, set_amount) = signal(0.0);
    //let (is_connected, set_is_connected) = signal(false);
    
    let WalletState{
        sol_address,
        eth_address,
        is_connected,
        ..
    }= use_context::<WalletState>().expect("WalletState context missing");



    //let rpc_url = "https://api.devnet.solana.com";
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
                sol_address.set("Phantom Wallet not detected".into());
                return;
            }
        };


        //get the connect
        let connect_fn = match unsafe {js_sys::Reflect::get(&solana, &JsValue::from_str("connect")) } {
            Ok(f) => f,
            Err(_) => {
                sol_address.set("Connect method not available".into());
                return;
            }
        };
    

        //convert to callable function
        let connect_fn = match connect_fn.dyn_into::<js_sys::Function>(){
            Ok(f) => f,
            Err(_) =>{
                sol_address.set("Invalid Connect function".into());
                return;
            }
        };

         // 5. Execute connection
         if let Err(e) = connect_fn.call0(&solana) {
            log::error!("Connection failed: {:?}", e);
            sol_address.set("Connection rejected".into());
            return;
        }

        //get the wallet public key 
        let pubkey = match  unsafe{js_sys::Reflect::get(&solana,&JsValue::from_str("publicKey"))}{
            Ok(pk) => pk,
            Err(_) => {
                sol_address.set("Failed to get public key".into());
                return;
            }
        };

        

        //convert it to make sure it is in string
        // Call `toString` on the publicKey JsValue
        let pubkey_str = match unsafe{js_sys::Reflect::get(&pubkey, &JsValue::from_str("toString"))} {
            Ok(to_string_fn) => {
                let to_string_fn = to_string_fn.dyn_into::<js_sys::Function>().unwrap();
                match to_string_fn.call0(&pubkey) {
                    Ok(val) => val.as_string().unwrap_or("Failed to stringify public key".to_string()),
                    Err(_) => "Failed to execute toString()".to_string(),
                }
            }
            Err(_) => "publicKey.toString() not available".to_string(),
        };

        //set_sol.set(pubkey_str); 
        sol_address.set(pubkey_str.into());
        is_connected.set(true);
    };


    let connect_ethereum = move |_| {

         // get the browser windows 
        let window = match web_sys::window(){
            Some(w) => w,
            None => {
                log::error!("windows object not found");
                return;
            }
        };
         
        //get the ethereum from the browser windows
        let ethereum = match window.get("ethereum"){
            Some(e) => e,
            None => {
                eth_address.set("Metamask Not found" .into());
                return;
            }
        };

        
        //get the jsValue object method 
        let connect_eth_fn =  match unsafe{js_sys::Reflect::get(&ethereum, &JsValue::from_str("request"))} {
            Ok(e) => e,
            Err(_) => {
                eth_address.set("Connection method not available".into());
                return;
            }
        };
        
        //make it a  rust function
        let connect_eth_fn =  match connect_eth_fn.dyn_into::<js_sys::Function>() {
            Ok(e) => e,
            Err(_) => {
                eth_address.set("Invalid Connection method".into());
                return;
            }
        };

        // 5. Execute connection
        if let Err(e) = connect_eth_fn.call1(&ethereum,&JsValue::from_str("eth_requestAccounts")) {
            log::error!("Connection failed: {:?}", e);
            eth_address.set("Connection rejected".into());
            return;
        }


        eth_address.set("0x892..4f1".into()); // Replace with real connection
    };
    



    view! {
        <div class="wallet-status">
            <p>"Network: Solana Devnet | Ethereum Sepolia"</p>
            <div>
                <h3> connect your wallet   </h3>
                <button on:click=connect_solana>
                    {move || format!("SOL: {}", sol_address.get())}
                </button>
                <button on:click=connect_ethereum>
                    {move || format!("ETH: {}", eth_address.get())}
                </button>
            </div>
        </div>
        <br/>
    }
}
