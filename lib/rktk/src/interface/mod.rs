pub mod backlight;
pub mod display;
pub mod double_tap;
pub mod error;
pub mod keyscan;
pub mod mouse;
pub mod split;
pub mod usb;

pub trait DriverBuilder {
    type Output;
    type Error;
    async fn build(self) -> Result<Self::Output, Self::Error>;
}
