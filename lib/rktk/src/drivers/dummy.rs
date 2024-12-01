use display_interface::DisplayError;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point},
    Pixel,
};
use rktk_keymanager::state::EncoderDirection;

use crate::drivers::interface::{
    backlight::BacklightDriver, ble::BleDriver, debounce::DebounceDriver, display::DisplayDriver,
    double_tap::DoubleTapResetDriver, encoder::EncoderDriver, error::RktkError, mouse::MouseDriver,
    reporter::ReporterDriver, split::SplitDriver, storage::StorageDriver, usb::UsbDriver,
    BackgroundTask, DriverBuilder, DriverBuilderWithTask,
};

// Backlight
pub enum Backlight {}
impl BacklightDriver for Backlight {
    async fn write<const N: usize>(&mut self, _colors: &[smart_leds::RGB8; N]) {
        unreachable!()
    }
}

// BLE
pub enum Ble {}
impl ReporterDriver for Ble {}
impl BleDriver for Ble {}

pub enum BleBuilder {}
impl DriverBuilderWithTask for BleBuilder {
    type Driver = Ble;

    type Error = ();

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Driver, BleTask), Self::Error> {
        unreachable!()
    }
}

pub enum BleTask {}
impl BackgroundTask for BleTask {
    async fn run(self) {
        unreachable!()
    }
}

// Debounce
pub enum Debounce {}
impl DebounceDriver for Debounce {
    fn should_ignore_event(
        &mut self,
        _: &rktk_keymanager::state::KeyChangeEvent,
        _: embassy_time::Instant,
    ) -> bool {
        unreachable!()
    }
}

// Display
pub enum Display {}
impl Dimensions for Display {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        unreachable!()
    }
}
impl DrawTarget for Display {
    type Color = BinaryColor;

    type Error = ();

    fn draw_iter<I>(&mut self, _pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        unreachable!()
    }
}
impl DisplayDriver for Display {
    const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build();
    const MAX_TEXT_WIDTH: usize = 10;

    fn clear_buffer(&mut self) {
        unreachable!()
    }
    async fn flush(&mut self) -> Result<(), DisplayError> {
        unreachable!()
    }
    fn calculate_point(_col: i32, _row: i32) -> Point {
        unreachable!()
    }
}

pub enum DisplayBuilder {}
impl DriverBuilder for DisplayBuilder {
    type Output = Display;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        unreachable!()
    }
}

// dtr
pub enum DoubleTapReset {}
impl DoubleTapResetDriver for DoubleTapReset {
    async fn execute(&self, _timeout: embassy_time::Duration) {
        unreachable!()
    }
}

// encoder
pub enum Encoder {}
impl EncoderDriver for Encoder {
    async fn read_wait(&mut self) -> (u8, EncoderDirection) {
        unreachable!()
    }
}

// mouse
pub enum Mouse {}
impl MouseDriver for Mouse {
    async fn read(&mut self) -> Result<(i8, i8), RktkError> {
        unreachable!()
    }
}

pub enum MouseBuilder {}
impl DriverBuilder for MouseBuilder {
    type Output = Mouse;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        unreachable!()
    }
}

// split
pub enum Split {}
impl SplitDriver for Split {
    async fn wait_recv(&mut self, _buf: &mut [u8], _is_master: bool) -> Result<(), RktkError> {
        unreachable!()
    }
    async fn send(&mut self, _buf: &[u8], _is_master: bool) -> Result<(), RktkError> {
        unreachable!()
    }
}

// storage
pub enum Storage {}
impl StorageDriver for Storage {
    type Error = ();
    async fn format(&self) -> Result<(), Self::Error> {
        unreachable!()
    }
    async fn read<const N: usize>(&self, _key: u64, _buf: &mut [u8]) -> Result<(), Self::Error> {
        unreachable!()
    }
    async fn write<const N: usize>(&self, _key: u64, _buf: &[u8]) -> Result<(), Self::Error> {
        unreachable!()
    }
}

// usb
pub enum Usb {}
impl ReporterDriver for Usb {}
impl UsbDriver for Usb {}

pub enum UsbBuilder {}
impl DriverBuilderWithTask for UsbBuilder {
    type Driver = Usb;

    type Error = ();

    #[allow(refining_impl_trait)]
    async fn build(self) -> Result<(Self::Driver, UsbTask), Self::Error> {
        unreachable!()
    }
}

pub enum UsbTask {}
impl BackgroundTask for UsbTask {
    async fn run(self) {
        unreachable!()
    }
}
