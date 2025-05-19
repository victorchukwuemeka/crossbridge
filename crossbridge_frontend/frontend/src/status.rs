use leptos::*;
use leptos::prelude::{ElementChild, ClassAttribute};


#[component]
pub fn NetworkStatus() -> impl IntoView {
    view! {
        <div class="status-panel">
        
            <h3>"Recent Transactions"</h3>
            <ul>
                <li>"◎5.0 → Ξ5.0 | ✅ Minted"</li>
                <li>"Ξ2.0 → ◎2.0 | 🔄 Processing"</li>
                <li>"◎1.5 → Ξ1.5 | ❌ Failed"</li>
            </ul>

            <h3>"Relayer Status"</h3>
            <p>"Last Block: Solana #184,292"</p>
            <p>"Latency: 12.3s avg"</p>
            <p>"Watchtowers: 5/5 online"</p>
        </div>
    }
}