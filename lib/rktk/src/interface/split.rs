use serde::{Deserialize, Serialize};

use super::{backlight::BacklightCtrl, error::RktkError};

pub trait SplitDriver {
    async fn init(&mut self) -> Result<(), RktkError> {
        Ok(())
    }
    async fn wait_recv(&mut self, buf: &mut [u8]) -> Result<(), RktkError>;
    async fn send(&mut self, buf: &[u8]) -> Result<(), RktkError>;
}

#[derive(Deserialize, Serialize, Debug)]
pub enum MasterToSlave {
    Backlight(BacklightCtrl),
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
