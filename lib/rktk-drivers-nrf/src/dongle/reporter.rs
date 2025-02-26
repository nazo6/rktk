use core::convert::Infallible;

use embassy_nrf::radio::Instance;
use embassy_nrf_esb::ptx::PtxRadio;
use rktk::drivers::interface::{
    ble::BleDriver,
    dongle::{DongleData, KeyboardReport},
    reporter::ReporterDriver,
    BackgroundTask, DriverBuilderWithTask,
};

pub struct EsbReporterDriverBuilder<T: Instance>(pub EsbReporterDriver<T>);

struct DummyTask;
impl BackgroundTask for DummyTask {
    async fn run(self) {}
}

impl<T: Instance> DriverBuilderWithTask for EsbReporterDriverBuilder<T> {
    type Driver = EsbReporterDriver<T>;

    type Error = ();

    async fn build(
        self,
    ) -> Result<
        (
            Self::Driver,
            impl rktk::drivers::interface::BackgroundTask + 'static,
        ),
        Self::Error,
    > {
        Ok((self.0, DummyTask))
    }
}

pub struct EsbReporterDriver<T: Instance> {
    ptx: PtxRadio<'static, T, 64>,
}

impl<T: Instance> EsbReporterDriver<T> {
    pub fn new(ptx: PtxRadio<'static, T, 64>) -> Self {
        Self { ptx }
    }
}

impl<T: Instance> ReporterDriver for EsbReporterDriver<T> {
    type Error = Infallible;

    fn try_send_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn try_send_media_keyboard_report(
        &self,
        _report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn try_send_mouse_report(
        &self,
        _report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    async fn send_keyboard_report(
        &mut self,
        report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        let report: KeyboardReport = report.into();
        let report = DongleData::Keyboard(report);
        let mut buf = [0u8; 64];
        if let Ok(data) = postcard::to_slice(&report, &mut buf) {
            self.ptx.send(0, data, false).await;
        }

        Ok(())
    }

    async fn send_mouse_report(
        &mut self,
        report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        let report = DongleData::Mouse(report.into());
        let mut buf = [0u8; 64];
        if let Ok(data) = postcard::to_slice(&report, &mut buf) {
            self.ptx.send(0, data, false).await;
        }

        Ok(())
    }
}

impl<T: Instance> BleDriver for EsbReporterDriver<T> {
    type Error = Infallible;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        todo!()
    }
}
