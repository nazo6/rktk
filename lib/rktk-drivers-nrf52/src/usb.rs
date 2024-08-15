use embassy_nrf::{
    peripherals::USBD,
    usb::{vbus_detect::SoftwareVbusDetect, Driver},
};
pub use rktk_drivers_common::usb::*;

pub fn new_usb(
    opts: UsbOpts<Driver<'static, USBD, &'static SoftwareVbusDetect>>,
) -> CommonUsbDriverBuilder<Driver<'static, USBD, &'static SoftwareVbusDetect>> {
    CommonUsbDriverBuilder::new(opts)
}
