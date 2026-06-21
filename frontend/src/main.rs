use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Chess Engines"</h1>
            <p>"Hello, World! Rust + WASM is live"</p>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
