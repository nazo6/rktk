#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use neg_nrf::start_master;
use rktk::{config::keymap::Keymap, hooks::create_empty_hooks};

// Empty keymap for demo.
// Please replace with your own keymap configuration.
static KM: Keymap = Keymap::const_default();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    start_master(create_empty_hooks(), &KM).await;
}
