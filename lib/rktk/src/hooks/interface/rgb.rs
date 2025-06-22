use crate::drivers::interface::rgb::*;

/// Hooks related to RGB functionality.
///
/// This trait allows for custom RGB behavior to be implemented.
/// These hooks are called in both master and slave sides of the keyboard.
pub trait RgbHooks: 'static {
    /// Invoked after the RGB driver is initialized.
    ///
    /// * `_driver`: [`RgbDriver`] instance to control RGB.
    /// * `_is_master`: If true, this is the master side of the keyboard.
    async fn on_rgb_init(&mut self, _driver: &mut impl RgbDriver, _is_master: bool) {}

    /// Invoked
    /// - after the [`RgbCommand`] is send and RGB task received it
    /// - before the RGB task processes the command.
    ///
    /// You can use this hook to modify the RGB mode before the RGB task processes.
    ///
    /// * `_driver`: [`RgbDriver`] instance to control RGB.
    /// * `_rgb_mode`: [`RgbMode`] to be processed.
    async fn on_rgb_process(&mut self, _driver: &mut impl RgbDriver, _rgb_mode: &mut RgbMode) {}
    async fn custom_rgb(&mut self, _driver: &mut impl RgbDriver, _brightness: f32) {}
}
