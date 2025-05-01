use leptos::*;
use solana_client_web::solana_sdk::pubkey::Pubkey;
use wasm_bindgen::prelude::*;
use web_sys::window;


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

    let connect_solana = move |_| {

        //get the browser window object
        let window = match  web_sys::window(){
            Some(w) => w,
            None => {
                log::error!("Window object not available");
                return;
            }
        };

        // get the solana 
        let solana = match window.get("solana"){
            Ok(s) => s,
            Err(_) => {
                set_sol.set("Phantom Wallet not detected".into());
                return;
            }
        };

        
        //get the connect 
        let connect_fn = match js_sys::Reflect::get(&solana, &JsValue::from_str("connect")){
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
            }
        };

         // 5. Execute connection
         if let Err(e) = connect_fn.call0(&solana) {
            log::error!("Connection failed: {:?}", e);
            set_sol.set("Connection rejected".into());
            return;
        }
        
        set_sol.set("FjGh3..8aK2".into()); // Replace with real connection
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
            Ok(e) => e,
            Err(_) => {
                set_eth.set("Metamask Not found" .into());
                return;
            }
        };

        

        let connect_eth_fn =  match js_sys::Reflect::get(&ethereum, &JsValue::from_str("request")) {
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
                    {move || format!("SOL: {}", sol_address())}
                </button>
                <button on:click=connect_ethereum>
                    {move || format!("ETH: {}", eth_address())}
                </button>
            </div>
        </div>
    }
}