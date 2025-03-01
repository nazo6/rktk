use core::convert::Infallible;

use embassy_nrf::{interrupt, radio::Instance, Peripheral};
pub use embassy_nrf_esb::ptx;
use embassy_nrf_esb::{
    ptx::{new_ptx, PtxConfig, PtxInterface, PtxTask},
    InterruptHandler, RadioConfig,
};
use postcard::experimental::max_size::MaxSize as _;
use rktk::drivers::interface::{
    ble::BleDriver, dongle::DongleData, reporter::ReporterDriver, BackgroundTask,
    DriverBuilderWithTask,
};

// -------- Builder ----------

pub struct EsbReporterDriverBuilder<T: Instance> {
    pub ptx_task: PtxTask<T, 255>,
    pub ptx_interface: PtxInterface,
}

impl<T: Instance> EsbReporterDriverBuilder<T> {
    pub fn new(
        radio: impl Peripheral<P = T> + 'static,
        irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'static,
        config: RadioConfig,
        ptx_config: PtxConfig,
    ) -> Self {
        let (ptx_task, ptx_interface) = new_ptx(radio, irq, config, ptx_config);

        Self {
            ptx_task,
            ptx_interface,
        }
    }
}

impl<T: Instance> DriverBuilderWithTask for EsbReporterDriverBuilder<T> {
    type Driver = EsbReporterDriver;

    type Error = ();

    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error> {
        Ok((
            EsbReporterDriver {
                ptx_interface: self.ptx_interface,
                cnt: core::sync::atomic::AtomicU8::new(0),
            },
            Task {
                ptx_task: self.ptx_task,
            },
        ))
    }
}

// --------- Task ----------

struct Task<T: Instance> {
    ptx_task: PtxTask<T, 255>,
}
impl<T: Instance> BackgroundTask for Task<T> {
    async fn run(mut self) {
        self.ptx_task.run().await;
    }
}

// ----------- Driver ------------

pub struct EsbReporterDriver {
    ptx_interface: PtxInterface,
    cnt: core::sync::atomic::AtomicU8,
}

#[derive(Debug)]
pub struct ErrorMsg(&'static str);
impl core::fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl core::error::Error for ErrorMsg {}

type DongleDataWithCnt = (usize, DongleData);

impl EsbReporterDriver {
    fn send_report(&self, report: DongleData) -> Result<(), ErrorMsg> {
        let mut buf = [0; DongleDataWithCnt::POSTCARD_MAX_SIZE];
        let cnt = self.cnt.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        postcard::to_slice(&(cnt, report), &mut buf).map_err(|_| ErrorMsg("ser"))?;
        self.ptx_interface
            .try_send(0, &buf, false)
            .map_err(|_| ErrorMsg("ch full"))?;
        Ok(())
    }
}

impl ReporterDriver for EsbReporterDriver {
    type Error = ErrorMsg;

    fn try_send_keyboard_report(
        &self,
        report: usbd_hid::descriptor::KeyboardReport,
    ) -> Result<(), Self::Error> {
        self.send_report(DongleData::Keyboard(report.into()))
    }

    fn try_send_media_keyboard_report(
        &self,
        report: usbd_hid::descriptor::MediaKeyboardReport,
    ) -> Result<(), Self::Error> {
        self.send_report(DongleData::MediaKeyboard(report.into()))
    }

    fn try_send_mouse_report(
        &self,
        report: usbd_hid::descriptor::MouseReport,
    ) -> Result<(), Self::Error> {
        self.send_report(DongleData::Mouse(report.into()))
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
