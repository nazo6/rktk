use embassy_time::Duration;

/// Driver to interact with the system
pub trait SystemDriver {
    /// Reboot the system if double-reset is detected.
    ///
    /// Implement this only for chips that require this feature (e.g. RP2040).
    /// There is no need to implement this if the feature is already implemented, such as the nRF52840 uf2 bootloader.
    async fn double_reset_usb_boot(&self, _timeout: Duration) {}

    fn reset(&self);

    /// Reset to the bootloader (typically uf2 flash mode)
    fn reset_to_bootloader(&self);
}
