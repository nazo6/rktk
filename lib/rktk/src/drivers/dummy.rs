//! Dummy drivers for type annotations.
//!
//! This is intended to be used by the [`crate::none_driver`] macro.

use core::convert::Infallible;

use display_interface::DisplayError;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point},
    Pixel,
};
use rktk_keymanager::state::EncoderDirection;

use crate::drivers::interface::{
    ble::BleDriver, debounce::DebounceDriver, display::DisplayDriver, encoder::EncoderDriver,
    mouse::MouseDriver, reporter::ReporterDriver, rgb::RgbDriver, split::SplitDriver,
    storage::StorageDriver, usb::UsbDriver, BackgroundTask, DriverBuilder, DriverBuilderWithTask,
};

// Rgb
pub enum Rgb {}
impl RgbDriver for Rgb {
    type Error = Infallible;
    async fn write<const N: usize>(
        &mut self,
        _colors: &[smart_leds::RGB8; N],
    ) -> Result<(), Self::Error> {
        unreachable!()
    }
}

// BLE
pub enum Ble {}
impl ReporterDriver for Ble {
    type Error = core::convert::Infallible;

    fn try_send_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn try_send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn try_send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        unreachable!()
    }
}
impl BleDriver for Ble {
    type Error = Infallible;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        unreachable!()
    }
}

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
    type Error = Infallible;

    async fn read(&mut self) -> Result<(i8, i8), Self::Error> {
        unreachable!()
    }

    async fn set_cpi(&mut self, _cpi: u16) -> Result<(), Self::Error> {
        unreachable!()
    }

    async fn get_cpi(&mut self) -> Result<u16, Self::Error> {
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
    type Error = Infallible;

    async fn wait_recv(&mut self, _buf: &mut [u8], _is_master: bool) -> Result<(), Self::Error> {
        unreachable!()
    }
    async fn send(&mut self, _buf: &[u8], _is_master: bool) -> Result<(), Self::Error> {
        unreachable!()
    }
}

// storage
pub enum Storage {}
impl StorageDriver for Storage {
    type Error = Infallible;
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
impl ReporterDriver for Usb {
    type Error = core::convert::Infallible;

    fn try_send_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn try_send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn try_send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        unreachable!()
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        unreachable!()
    }
}
impl UsbDriver for Usb {
    type Error = Infallible;

    async fn vbus_detect(&self) -> Result<bool, <Self as UsbDriver>::Error> {
        unreachable!()
    }
}

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
