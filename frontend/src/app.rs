use leptos::*;
use wallet::WalletStatus;
use bridge::BridgeActions;
use status::NetworkStatus;

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
            <p>"Bridge Security: Audited by Halborn (2023-12-01)"</p>
            <p>"Version: v1.2.0 | Relayer v3.1.4"</p>
        </footer>
    }
}