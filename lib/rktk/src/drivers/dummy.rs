//! Dummy drivers just for type annotations.
//!
//! This is intended to be used by the [`crate::none_driver`] macro.

use core::convert::Infallible;

use display_interface::DisplayError;
use embedded_graphics::{
    Pixel,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, Point},
};
use rktk_keymanager::interface::state::input_event::{EncoderDirection, KeyChangeEvent};

use crate::drivers::interface::{
    ble::BleDriver,
    debounce::DebounceDriver,
    display::DisplayDriver,
    encoder::EncoderDriver,
    mouse::MouseDriver,
    reporter::ReporterDriver,
    rgb::RgbDriver,
    split::{SplitDriver, SplitDriverBuilder},
    storage::StorageDriver,
    usb::UsbDriver,
};

use super::interface::{
    ble::BleDriverBuilder, display::DisplayDriverBuilder, mouse::MouseDriverBuilder,
    usb::UsbDriverBuilder,
};

// Rgb
pub fn rgb() -> Option<impl RgbDriver> {
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

    Option::<Rgb>::None
}

// Debounce
pub fn debounce() -> Option<impl DebounceDriver> {
    pub enum Debounce {}
    impl DebounceDriver for Debounce {
        fn should_ignore_event(&mut self, _: &KeyChangeEvent, _: embassy_time::Instant) -> bool {
            unreachable!()
        }
    }

    Option::<Debounce>::None
}

// Display
pub fn display_builder() -> Option<impl DisplayDriverBuilder> {
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
        const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new().build();
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

    impl DisplayDriverBuilder for DisplayBuilder {
        type Output = Display;
        type Error = ();

        async fn build(self) -> Result<Self::Output, Self::Error> {
            unreachable!()
        }
    }

    Option::<DisplayBuilder>::None
}

// encoder
pub fn encoder() -> Option<impl EncoderDriver> {
    pub enum Encoder {}
    impl EncoderDriver for Encoder {
        async fn read_wait(&mut self) -> (u8, EncoderDirection) {
            unreachable!()
        }
    }

    Option::<Encoder>::None
}

// mouse
pub fn mouse_builder() -> Option<impl MouseDriverBuilder> {
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
    impl MouseDriverBuilder for MouseBuilder {
        type Output = Mouse;
        type Error = Infallible;

        async fn build(self) -> Result<Self::Output, Self::Error> {
            unreachable!()
        }
    }
    Option::<MouseBuilder>::None
}

// split
pub fn split_builder() -> Option<impl SplitDriverBuilder> {
    pub enum Split {}
    impl SplitDriver for Split {
        type Error = Infallible;

        async fn recv(&mut self, _buf: &mut [u8], _is_master: bool) -> Result<usize, Self::Error> {
            unreachable!()
        }
        async fn send_all(&mut self, _buf: &[u8], _is_master: bool) -> Result<(), Self::Error> {
            unreachable!()
        }
    }
    pub enum SplitBuilder {}
    impl SplitDriverBuilder for SplitBuilder {
        type Output = Split;
        type Error = Infallible;

        async fn build(self) -> Result<Self::Output, Self::Error> {
            unreachable!()
        }
    }

    Option::<SplitBuilder>::None
}

// storage
pub fn storage() -> Option<impl StorageDriver> {
    pub enum Storage {}
    impl StorageDriver for Storage {
        type Error = Infallible;
        async fn format(&self) -> Result<(), Self::Error> {
            unreachable!()
        }
        async fn read<const N: usize>(
            &self,
            _key: u64,
            _buf: &mut [u8],
        ) -> Result<(), Self::Error> {
            unreachable!()
        }
        async fn write<const N: usize>(&self, _key: u64, _buf: &[u8]) -> Result<(), Self::Error> {
            unreachable!()
        }
    }

    Option::<Storage>::None
}

// Reporter

use usbd_hid::descriptor::*;
pub enum DummyReporter {}
impl ReporterDriver for DummyReporter {
    type Error = core::convert::Infallible;

    fn try_send_keyboard_report(&self, _report: KeyboardReport) -> Result<(), Self::Error> {
        unreachable!()
    }
    fn try_send_media_keyboard_report(
        &self,
        _report: MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        unreachable!()
    }
    fn try_send_mouse_report(&self, _report: MouseReport) -> Result<(), Self::Error> {
        unreachable!()
    }
    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        unreachable!()
    }
    fn wakeup(&self) -> Result<bool, Self::Error> {
        unreachable!()
    }
}
impl BleDriver for DummyReporter {
    type Error = Infallible;
    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        unreachable!()
    }
}
impl UsbDriver for DummyReporter {
    type Error = Infallible;

    async fn vbus_detect(&self) -> Result<bool, <Self as UsbDriver>::Error> {
        unreachable!()
    }
}

// BLE
pub fn ble_builder() -> Option<impl BleDriverBuilder> {
    pub enum BleBuilder {}
    impl BleDriverBuilder for BleBuilder {
        type Output = DummyReporter;
        type Error = Infallible;
        #[allow(refining_impl_trait)]
        async fn build(self) -> Result<(Self::Output, futures::future::Pending<()>), Self::Error> {
            unreachable!()
        }
    }

    Option::<BleBuilder>::None
}

// usb
pub fn usb_builder() -> Option<impl UsbDriverBuilder> {
    pub enum UsbBuilder {}
    impl UsbDriverBuilder for UsbBuilder {
        type Output = DummyReporter;
        type Error = ();

        #[allow(refining_impl_trait)]
        async fn build(self) -> Result<(Self::Output, futures::future::Pending<()>), Self::Error> {
            unreachable!()
        }
    }

    Option::<UsbBuilder>::None
}
