#![no_std]
#![no_main]

use embassy_executor::Spawner;
use magnetic_keypad_nrf::{run, keymap::KEYMAP};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    run(spawner, &KEYMAP).await;
}
