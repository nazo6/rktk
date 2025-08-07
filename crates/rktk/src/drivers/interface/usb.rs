use super::reporter::ReporterDriver;

pub trait UsbReporterDriver: ReporterDriver {
    type Error: super::Error;

    async fn vbus_detect(&self) -> Result<bool, <Self as UsbReporterDriver>::Error>;
}

super::generate_builder!(UsbReporterDriver);
