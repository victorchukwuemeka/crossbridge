use leptos::prelude::*;

mod app;
mod wallet;
mod bridge;
mod status;


fn main() {
    leptos::mount::mount_to_body(app::App);
}