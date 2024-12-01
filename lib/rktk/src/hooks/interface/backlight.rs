use crate::drivers::interface::backlight::{BacklightCommand, BacklightDriver};

pub use smart_leds::RGB8;

pub trait BacklightHooks {
    async fn on_backlight_init(&mut self, _driver: &mut impl BacklightDriver) {}
    async fn on_backlight_process<const N: usize>(
        &mut self,
        _driver: &mut impl BacklightDriver,
        _command: &BacklightCommand,
        _rgb_data: &mut Option<[RGB8; N]>,
    ) {
    }
}
