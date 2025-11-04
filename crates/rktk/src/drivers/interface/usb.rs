use super::reporter::ReporterDriver;

pub trait UsbReporterDriver: ReporterDriver {
    type Error: super::Error;

    async fn vbus_detect(&self);
}

super::generate_builder!(UsbReporterDriver);
