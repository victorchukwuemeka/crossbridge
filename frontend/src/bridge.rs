use leptos::*;
use web_sys::console;

#[component]
pub fn BridgeActions() -> impl IntoView {
    let (action, set_action) = create_signal("lock");
    let (amount, set_amount) = create_signal(0.0);
    let (is_loading, set_loading) = create_signal(false);

    let submit = move |_| {
        set_loading.set(true);
        let amount = amount.get();
        
        match action.get().as_str() {
            "lock" => {
                console::log_1(&format!("Locking {} SOL", amount).into());
                // Add real Solana TX here
            },
            _ => {
                console::log_1(&format!("Burning {} WSOL", amount).into());
                // Add real Ethereum TX here
            }
        }
        
        set_loading.set(false);
    };

    view! {
        <div class="action-panel">
            <select on:change=move |ev| set_action.set(event_target_value(&ev))>
                <option value="lock">"Lock SOL → Mint WSOL"</option>
                <option value="burn">"Burn WSOL → Unlock SOL"</option>
            </select>

            <input
                type="number"
                placeholder=move || if action.get() == "lock" { "◎ Amount" } else { "Ξ Amount" }
                on:input=move |ev| set_amount.set(event_target_value(&ev).parse().unwrap_or(0.0))
            />

            <button 
                on:click=submit
                disabled=is_loading
            >
                {move || match (action.get().as_str(), is_loading.get()) {
                    ("lock", false) => "Lock & Mint WSOL",
                    ("lock", true) => "Processing...",
                    (_, false) => "Burn & Unlock SOL",
                    (_, true) => "Processing...",
                }}
            </button>

            <Show when=move || action.get() == "lock">
                <p>"Min: ◎0.1 | 1:1 Peg"</p>
            </Show>
            <Show when=move || action.get() == "burn">
                <p>"Unlock Time: ~5 min"</p>
            </Show>
        </div>
    }
}