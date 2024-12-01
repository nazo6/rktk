use serde::{Deserialize, Serialize};

use super::backlight::BacklightCommand;

pub trait SplitDriver {
    type Error: core::error::Error;

    async fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn wait_recv(&mut self, buf: &mut [u8], is_master: bool) -> Result<(), Self::Error>;
    async fn send(&mut self, buf: &[u8], is_master: bool) -> Result<(), Self::Error>;
}

#[derive(Deserialize, Serialize, Debug)]
pub enum MasterToSlave {
    Backlight(BacklightCommand),
    Message(u8),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SlaveToMaster {
    Pressed(u8, u8),
    Released(u8, u8),
    Mouse { x: i8, y: i8 },
    Message(u8),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum KeyChangeType {
    Pressed,
    Released,
}
