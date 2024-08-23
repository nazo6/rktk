#![cfg_attr(not(feature = "std"), no_std)]
// without this rust-analyzer show warning like:
// Function `__wbg_instanceof_JsType...` should have snake_case name, e.g. `__wbg_instanceof_js_type_...`
// Maybe this is because of tsify's macro implementation problem.
#![allow(non_snake_case)]

#[cfg(feature = "client")]
pub mod client;
pub mod endpoints;
#[cfg(feature = "server")]
pub mod server;

#[doc(hidden)]
pub mod __reexports {
    pub use futures;
    pub use heapless;
    pub use postcard;
}
