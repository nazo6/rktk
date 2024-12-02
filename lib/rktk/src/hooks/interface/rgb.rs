use crate::drivers::interface::rgb::{RgbCommand, RgbDriver};

pub use smart_leds::RGB8;

pub trait RgbHooks {
    async fn on_rgb_init(&mut self, _driver: &mut impl RgbDriver) {}
    async fn on_rgb_process<const N: usize>(
        &mut self,
        _driver: &mut impl RgbDriver,
        _command: &RgbCommand,
        _rgb_data: &mut Option<[RGB8; N]>,
    ) {
    }
}
