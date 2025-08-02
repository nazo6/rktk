#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use keyball61_nrf::start;
use rktk::config::keymap::Keymap;

static KM: Keymap = Keymap::const_default();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    start(spawner, &KM).await;
}
