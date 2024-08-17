#![cfg_attr(not(feature = "std"), no_std)]

use postcard::experimental::max_size::MaxSize;
use rktk_keymanager::keycode::KeyDef;

pub mod endpoints;
pub use futures;
pub mod server;

#[derive(MaxSize)]
pub struct UpdateKey {
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub key: KeyDef,
}
