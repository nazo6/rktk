use core::sync::atomic::AtomicBool;

use embassy_futures::select::{select, Either};
use embassy_rp::{peripherals::USB, usb::Driver};
pub use embassy_usb::Config as UsbConfig;
use embassy_usb::UsbDevice;
pub use rktk_drivers_common::usb::interface::*;
use rktk_drivers_common::usb::{driver::CommonUsbDriver, RemoteWakeupSignal};

pub static SUSPENDED: AtomicBool = AtomicBool::new(false);

pub async fn new_usb(
    user_opts: UsbUserOpts<'static>,
    driver: Driver<'static, USB>,
) -> CommonUsbDriver<Driver<'static, USB>> {
    CommonUsbDriver::create_and_start(user_opts, driver, start_usb).await
}

#[embassy_executor::task]
async fn start_usb(
    mut device: UsbDevice<'static, Driver<'static, USB>>,
    signal: &'static RemoteWakeupSignal,
) {
    loop {
        device.run_until_suspend().await;
        match select(device.wait_resume(), signal.wait()).await {
            Either::First(_) => {}
            Either::Second(_) => {
                // ref: https://github.com/rp-rs/rp-hal/blob/a1b20f3a2cc0702986c478b0e1ee666f44d66853/rp2040-hal/src/usb.rs#L494
                // https://docs.rs/rp-pac/6.0.0/rp_pac/usb/regs/struct.SieCtrl.html
                //
                // TODO: クリティカルセッションのことを考慮しないとだめかも？
                // まあそんなに同時に書き込まれることはないだろうしとりあえず…
                embassy_rp::pac::USBCTRL_REGS
                    .sie_ctrl()
                    .modify(|w| w.set_resume(true));
            }
        }
    }
}
