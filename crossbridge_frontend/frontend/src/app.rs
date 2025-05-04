use leptos::*;
use crate::wallet::WalletStatus;
use crate::bridge::BridgeActions;
use crate::status::NetworkStatus;
use leptos::prelude::*;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <header>
            <h1>"SOLANA ⇄ ETHEREUM BRIDGE"</h1>
           <WalletStatus/>
        </header>

        <main class="grid-2-col">
            <BridgeActions/>
            <NetworkStatus/>
        </main>

        <footer>
            <p>"Bridge Security: Audited by victor (2023-12-01)"</p>
            <p>"Version: v1.2.0 | Relayer v3.1.4"</p>
        </footer>
    }
}