use rktk_log::derive_format_and_debug;
use serde::{Deserialize, Serialize};

use super::rgb::RgbCommand;

pub trait SplitDriver: 'static {
    type Error: core::error::Error;

    async fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Receive data from the other side and return the number of bytes received.
    ///
    /// If there is no data, this function should wait until data is received.
    async fn recv(&mut self, buf: &mut [u8], is_master: bool) -> Result<usize, Self::Error>;

    /// Send data to the other side.
    ///
    /// Implemention should wait until the *all* data is sent.
    async fn send_all(&mut self, buf: &[u8], is_master: bool) -> Result<(), Self::Error>;
}

#[derive_format_and_debug]
#[derive(Deserialize, Serialize)]
pub enum MasterToSlave {
    Rgb(RgbCommand),
    Message(u8),
}

#[derive_format_and_debug]
#[derive(Deserialize, Serialize)]
pub enum SlaveToMaster {
    Pressed(u8, u8),
    Released(u8, u8),
    Mouse { x: i8, y: i8 },
    Message(u8),
}

#[derive_format_and_debug]
#[derive(Deserialize, Serialize)]
pub enum KeyChangeType {
    Pressed,
    Released,
}
