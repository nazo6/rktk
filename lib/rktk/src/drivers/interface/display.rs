use display_interface::DisplayError;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};

/// Interface for display drivers.
///
/// TODO: Allow sync-only drivers?
pub trait DisplayDriver: AsRef<Self::Display> + AsMut<Self::Display> + 'static {
    type Display: DrawTarget<Color = BinaryColor>;

    /// Called when the display is initialized.
    ///
    /// It is guaranteed that:
    /// - No other function is called before this function.
    /// - If this function returns an error, other functions will not be called.
    ///
    /// Default implementation returns `Ok(())`.
    async fn init(&mut self) -> Result<(), DisplayError> {
        Ok(())
    }

    async fn flush(&mut self) -> Result<(), DisplayError> {
        Ok(())
    }

    /// Sets brightness of the display.
    ///
    /// 0 is off, 255 is full brightness.
    async fn set_brightness(&mut self, _brightness: u8) -> Result<(), DisplayError> {
        Err(DisplayError::DataFormatNotImplemented)
    }

    async fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        if on {
            self.set_brightness(255).await
        } else {
            self.set_brightness(0).await
        }
    }
}
