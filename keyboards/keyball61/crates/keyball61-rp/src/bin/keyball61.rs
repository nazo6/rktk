#![no_std]
#![no_main]

use embassy_executor::Spawner;
use keyball61_rp::start;
use rktk::config::keymap::Keymap;

static KM: Keymap = Keymap::const_default();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    start(spawner, &KM).await;
}
