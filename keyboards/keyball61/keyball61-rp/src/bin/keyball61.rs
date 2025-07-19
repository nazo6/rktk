#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use keyball61_rp::start;
use rktk::config::keymap::Keymap;

static KM: Keymap = Keymap::const_default();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    start(&KM).await;
}
