pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod tables;
#[cfg(feature = "ssr")]
pub mod state;
pub mod auth;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

#[cfg(feature = "ssr")]
pub mod ex {

    #[derive(Clone, Debug)]
    pub struct ReqInfo {
        pub url: String
    }
}