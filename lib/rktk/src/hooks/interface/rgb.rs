use crate::drivers::interface::rgb::{RgbDriver, RgbMode};

pub trait RgbHooks: 'static {
    async fn on_rgb_init(&mut self, _driver: &mut impl RgbDriver) {}
    async fn on_rgb_process(&mut self, _driver: &mut impl RgbDriver, _rgb_mode: &mut RgbMode) {}
}
