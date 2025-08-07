#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use neg_nrf::{init_peri, start_master};
use rktk::{config::keymap::Keymap, hooks::create_empty_hooks};

// Empty keymap for demo.
// Please replace with your own keymap configuration.
static KM: Keymap = Keymap::const_default();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = init_peri();
    start_master(spawner, p, create_empty_hooks(), &KM).await;
}
