#![feature(fn_traits)]

pub mod app;
pub mod components;
pub mod models;
pub mod repositories;

use cfg_if::cfg_if;

// cfg_if! is a macro that allows you to define code that depends on a large number of
// configuration options in a more concise way than using if/else blocks.
cfg_if! {
    if #[cfg(feature = "hydrate")] {

        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
            use app::*;
            console_error_panic_hook::set_once();
            leptos::mount_to_body(App);
        }
    }
}
