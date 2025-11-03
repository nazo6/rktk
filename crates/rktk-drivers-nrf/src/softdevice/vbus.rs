use embassy_executor::Spawner;
use embassy_nrf::usb::vbus_detect::{SoftwareVbusDetect, VbusDetect};
use nrf_softdevice::SocEvent;
use rktk::utils::Signal;

pub struct SoftdeviceVbusDetect(SoftwareVbusDetect);

#[embassy_executor::task]
async fn sd_vbus_task(d: &'static SoftdeviceVbusDetect, signal: &'static Signal<SocEvent>) {
    loop {
        match signal.wait().await {
            SocEvent::PowerUsbRemoved => d.0.detected(false),
            SocEvent::PowerUsbPowerReady => d.0.ready(),
            SocEvent::PowerUsbDetected => d.0.detected(true),
            _ => {}
        }
    }
}

impl SoftdeviceVbusDetect {
    /// Initialize the SoftdeviceVbusDetect and return a static reference to it.
    ///
    /// This function must be called only once.
    pub fn init(spawner: Spawner, signal: &'static Signal<SocEvent>) -> &'static Self {
        static VBUS_DETECT: static_cell::StaticCell<SoftdeviceVbusDetect> =
            static_cell::StaticCell::new();

        let d = Self(SoftwareVbusDetect::new(false, false));
        let d = VBUS_DETECT.init(d);

        spawner.must_spawn(sd_vbus_task(d, signal));

        d
    }
}

impl VbusDetect for &SoftdeviceVbusDetect {
    fn is_usb_detected(&self) -> bool {
        VbusDetect::is_usb_detected(&&self.0)
    }

    async fn wait_power_ready(&mut self) -> Result<(), ()> {
        VbusDetect::wait_power_ready(&mut &self.0).await
    }
}
