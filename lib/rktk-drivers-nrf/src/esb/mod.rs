pub use esb_ng::{Addresses, ConfigBuilder, Error};

pub mod dongle;
pub mod reporter;

#[derive(Default)]
pub struct Config {
    pub addresses: Addresses,
    pub config: ConfigBuilder,
}

pub fn create_address(channel: u8) -> Result<Addresses, Error> {
    Addresses::new(
        [0xE7, 0xE7, 0xE7, 0xE7],
        [0xC2, 0xC2, 0xC2, 0xC2],
        [0xE7, 0xC2, 0xC3, 0xC4],
        [0xC5, 0xC6, 0xC7, 0xC8],
        channel,
    )
}
