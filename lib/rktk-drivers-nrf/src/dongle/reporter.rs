use core::convert::Infallible;

use embassy_nrf::radio::Instance;
use embassy_nrf_esb::ptx::PtxRadio;
use rktk::{
    drivers::interface::{
        ble::BleDriver, dongle::DongleData, reporter::ReporterDriver, BackgroundTask,
        DriverBuilderWithTask,
    },
    utils::Channel,
};

static SEND_CHAN: Channel<DongleData, 64> = Channel::new();

// -------- Builder ----------

pub struct EsbReporterDriverBuilder<T: Instance> {
    ptx: PtxRadio<'static, T, 64>,
}

impl<T: Instance> EsbReporterDriverBuilder<T> {
    pub fn new(ptx: PtxRadio<'static, T, 64>) -> Self {
        Self { ptx }
    }
}

impl<T: Instance> DriverBuilderWithTask for EsbReporterDriverBuilder<T> {
    type Driver = EsbReporterDriver;

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
        Ok((EsbReporterDriver {}, Task { ptx: self.ptx }))
    }
}

// --------- Task ----------

struct Task<T: Instance> {
    ptx: PtxRadio<'static, T, 64>,
}
impl<T: Instance> BackgroundTask for Task<T> {
    async fn run(mut self) {
        loop {
            let data = SEND_CHAN.receive().await;
            let mut buf = [0u8; 64];
            if let Ok(data) = postcard::to_slice(&data, &mut buf) {
                // rktk::print!("{}:{:?}", data.len(), embassy_time::Instant::now());
                self.ptx.send(0, data, false).await;
            }
        }
    }
}

// ----------- Driver ------------

pub struct EsbReporterDriver {}

impl ReporterDriver for EsbReporterDriver {
    type Error = Infallible;

    fn try_send_keyboard_report(
        &self,
        report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        SEND_CHAN.try_send(DongleData::Keyboard(report.into()));
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
        report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        SEND_CHAN.try_send(DongleData::Mouse(report.into()));

        Ok(())
    }

    async fn send_rrp_data(&self, _data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn wakeup(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

impl BleDriver for EsbReporterDriver {
    type Error = Infallible;

    async fn clear_bond_data(&self) -> Result<(), <Self as BleDriver>::Error> {
        Ok(())
    }
}
