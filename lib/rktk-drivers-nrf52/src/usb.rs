pub mod handler;

use core::sync::atomic::AtomicBool;

use embassy_futures::select::{select, Either};
use embassy_nrf::{
    peripherals::USBD,
    usb::{vbus_detect::SoftwareVbusDetect, Driver},
};
pub use embassy_usb::Config as UsbConfig;
use embassy_usb::UsbDevice;
pub use rktk_drivers_common::usb::interface::*;
use rktk_drivers_common::usb::{driver::CommonUsbDriver, RemoteWakeupSignal};

pub static SUSPENDED: AtomicBool = AtomicBool::new(false);

pub async fn new_usb(
    user_opts: UsbUserOpts<'static>,
    driver: Driver<'static, USBD, &'static SoftwareVbusDetect>,
) -> CommonUsbDriver<Driver<'static, USBD, &'static SoftwareVbusDetect>> {
    CommonUsbDriver::create_and_start(user_opts, driver, start_usb).await
}

#[embassy_executor::task]
async fn start_usb(
    mut device: UsbDevice<'static, Driver<'static, USBD, &'static SoftwareVbusDetect>>,
    signal: &'static RemoteWakeupSignal,
) {
    loop {
        device.run_until_suspend().await;
        match select(device.wait_resume(), signal.wait()).await {
            Either::First(_) => {}
            Either::Second(_) => {
                // embassy_rp::pac::USBCTRL_REGS
                //     .sie_ctrl()
                //     .modify(|w| w.set_resume(true));
            }
        }
    }
}
