#![no_std]
#![no_main]

use embassy_executor::Spawner;
use magnetic_keypad_nrf::{keymap::KEYMAP, run};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    run(spawner, &KEYMAP).await;
}
