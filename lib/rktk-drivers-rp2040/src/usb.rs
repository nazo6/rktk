use embassy_rp::{peripherals::USB, usb::Driver};
pub use rktk_drivers_common::usb::*;

pub fn new_usb(
    opts: UsbOpts<Driver<'static, USB>>,
) -> CommonUsbDriverBuilder<Driver<'static, USB>> {
    CommonUsbDriverBuilder::new(opts)
}
