use leptos::*;
use leptos::prelude::RwSignal;
use leptos::prelude::create_rw_signal;

#[derive(Copy, Clone)]
pub struct WalletState {
    pub sol_address: RwSignal<String>,
    pub eth_address: RwSignal<String>,
    pub amount: RwSignal<f64>,
    pub is_connected: RwSignal<bool>,
}



// Optional: Add constructor/helper methods
impl WalletState {
    pub fn new() -> Self {
        Self {
            sol_address: create_rw_signal("Not connected".to_string()),
            eth_address: create_rw_signal("Not connected".to_string()),
            amount: create_rw_signal(0.0),
            is_connected: create_rw_signal(false),
        }
    }
}