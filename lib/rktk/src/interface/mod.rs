//! This module contains the interface for the drivers.
#![allow(async_fn_in_trait)]

pub mod backlight;
pub mod ble;
pub mod debounce;
pub mod display;
pub mod double_tap;
pub mod encoder;
pub mod error;
pub mod keyscan;
pub mod mouse;
pub mod rand;
pub mod reporter;
pub mod split;
pub mod storage;
pub mod usb;

pub trait DriverBuilder {
    type Output;
    type Error: core::fmt::Debug;
    async fn build(self) -> Result<Self::Output, Self::Error>;
}

pub trait DriverBuilderWithTask {
    type Driver;
    type Error: core::fmt::Debug;
    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask), Self::Error>;
}

pub trait BackgroundTask {
    async fn run(self);
}
