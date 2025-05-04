use leptos::*;
use web_sys::console;
//use leptos::prelude::{ElementChild, ClassAttribute};
use leptos::prelude::{Set, ElementChild, OnAttribute, ClassAttribute, Get};
use leptos::prelude::signal;
use crate::event_target_value;
use crate::Show;
//use leptos::prelude::event_target_value;

#[component]
pub fn BridgeActions() -> impl IntoView {
    let (action, set_action) = signal("lock".to_string());
    let (amount, set_amount) = signal(0.0);
    let (is_loading, set_loading) = signal(false);

    let submit = move |_| {
        set_loading.set(true);
        let amount = amount.get();
        
        match action.get().as_ref() {
            "lock" => {
                unsafe {console::log_1(&format!("Locking {} SOL", amount).into())};
                // Add real Solana TX here
            },
            _ => {
               unsafe{console::log_1(&format!("Burning {} WSOL", amount).into())};
                // Add real Ethereum TX here
            }
        }
        
        set_loading.set(false);
    };

    view! {
        <div class="action-panel">
            
            <select on:change=move |ev| {
                set_action.set(event_target_value(&ev)); 
            }>
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
                {move || match (action.get().as_ref(), is_loading.get()) {
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