//! Driver interface types
#![allow(async_fn_in_trait)]

pub mod ble;
pub mod debounce;
pub mod display;
pub mod dongle;
pub mod encoder;
pub mod keyscan;
pub mod mouse;
pub mod reporter;
pub mod rgb;
pub mod split;
pub mod storage;
pub mod system;
pub mod usb;

pub trait DriverBuilder {
    type Output;
    type Error: core::fmt::Debug;
    async fn build(self) -> Result<Self::Output, Self::Error>;
}

pub trait DriverBuilderWithTask {
    type Driver;
    type Error: core::fmt::Debug;
    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error>;
}

pub trait BackgroundTask {
    async fn run(self);
}
