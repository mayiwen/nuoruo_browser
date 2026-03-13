mod app;
pub mod page;
use app::*;
use leptos::prelude::*;
pub mod models;
pub mod myw;
pub mod util;
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>

        }
    })
}
