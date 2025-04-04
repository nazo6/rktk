use super::reporter::ReporterDriver;

pub trait UsbDriver: ReporterDriver {
    type Error: core::error::Error;

    async fn vbus_detect(&self) -> Result<bool, <Self as UsbDriver>::Error>;
}

super::generate_builder!(UsbDriver);
