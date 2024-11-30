use serde::{Deserialize, Serialize};

use super::{backlight::BacklightCommand, error::RktkError};

pub trait SplitDriver {
    async fn init(&mut self) -> Result<(), RktkError> {
        Ok(())
    }
    async fn wait_recv(&mut self, buf: &mut [u8], is_master: bool) -> Result<(), RktkError>;
    async fn send(&mut self, buf: &[u8], is_master: bool) -> Result<(), RktkError>;
}

/// Dummy driver.
/// This driver has two usage:
/// 1. Just a type notation. ex: `Option::<DummySplitDriver>::None`
/// 2. Use as a driver for testing: `let mut driver = DummySplitDriver;`
pub struct DummySplitDriver;
impl SplitDriver for DummySplitDriver {
    async fn wait_recv(&mut self, _buf: &mut [u8], _is_master: bool) -> Result<(), RktkError> {
        loop {
            embassy_time::Timer::after_secs(1000000).await;
        }
    }
    async fn send(&mut self, _buf: &[u8], _is_master: bool) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
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
