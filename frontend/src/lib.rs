//! Crap Todo -- a kanban task manager built on Crap CMS.
//!
//! Leptos SSR + WASM hydration frontend. All data flows through the
//! Crap CMS gRPC API; this crate contains no database logic.

#![recursion_limit = "1024"]

pub mod api;
pub mod app;
pub mod generated;
pub mod prelude;
#[cfg(feature = "ssr")]
#[allow(dead_code)]
pub mod generated_proto;
pub mod types;

#[cfg(feature = "ssr")]
pub mod grpc;

#[cfg(feature = "ssr")]
pub mod sse;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);
}
