use super::{error::RktkError, reporter::ReporterDriver};

pub trait UsbDriver: ReporterDriver {
    async fn vbus_detect(&self) -> Result<bool, RktkError> {
        Err(RktkError::NotSupported)
    }
}
