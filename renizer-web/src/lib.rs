pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod g_state;
pub mod tables;

#[cfg(feature = "ssr")]
use once_cell::sync::Lazy;

#[cfg(feature = "ssr")]
#[allow(unused)]
static G_STATE: Lazy<g_state::GState> = Lazy::new(|| g_state::GState::default()); 

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
