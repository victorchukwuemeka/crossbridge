use leptos::*;
use wasm_bindgen::prelude::*;

mod app;
mod wallet;
mod bridge;
mod status;

#[wasm_bindgen]
pub fn run_app() {
    mount_to_body(app::App);
}