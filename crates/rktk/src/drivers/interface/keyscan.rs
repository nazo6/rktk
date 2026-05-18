pub use kmsm::interface::state::input_event::KeyChangeEvent;

/// Key scanner driver interface.
///
/// The keyscan driver has two roles:
/// - Scanning the keys
/// - Determining which hand is currently using the keyboard on a split keyboard
///
/// This is because the key scanning circuit often includes a left/right determination circuit.
pub trait KeyscanDriver {
    /// The size of the calibration data in bytes. If 0, calibration is not supported.
    const CALIBRATION_SIZE: usize = 0;

    type CalibrationError: core::error::Error;

    /// Save calibration data into the provided buffer. Returns `Ok(())` on success.
    fn save_calibration(&self, _buf: &mut [u8]) -> Result<(), Self::CalibrationError> {
        Ok(())
    }

    /// Load calibration data from the provided buffer. Returns `Ok(())` on success.
    fn load_calibration(&mut self, _buf: &[u8]) -> Result<(), Self::CalibrationError> {
        Ok(())
    }

    /// Scans a key and returns the delta from the previous key scan
    async fn scan(&mut self, callback: impl FnMut(KeyChangeEvent));

    /// Starts calibration mode.
    fn start_calibration(&mut self) {}

    /// Ends calibration mode.
    fn end_calibration(&mut self) {}
}
