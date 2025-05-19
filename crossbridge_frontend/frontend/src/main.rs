use leptos::prelude::*;

mod app;
mod wallet;
mod bridge;
mod status;
mod models;


fn main() {
    leptos::mount::mount_to_body(app::App);
    //leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}