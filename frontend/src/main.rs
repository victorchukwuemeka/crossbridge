use leptos::*;
use wasm_bindgen::prelude::*;

use leptos::mount::mount_to_body;

mod app;
mod wallet;
mod bridge;
mod status;


fn main() {
    mount_to_body(App);
}