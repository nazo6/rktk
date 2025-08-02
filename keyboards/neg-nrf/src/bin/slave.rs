#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use embassy_executor::Spawner;
use neg_nrf::{init_peri, start_slave};
use rktk::hooks::create_empty_hooks;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = init_peri();
    start_slave(p, create_empty_hooks()).await;
}
