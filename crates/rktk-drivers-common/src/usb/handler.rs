use core::sync::atomic::Ordering;
use embassy_usb::Handler;

use super::SUSPENDED;

pub struct UsbDeviceHandler {}

impl UsbDeviceHandler {
    pub fn new() -> Self {
        UsbDeviceHandler {}
    }
}

// 参考: https://www.itf.co.jp/tech/road-to-usb-master/usb-status
impl Handler for UsbDeviceHandler {
    fn enabled(&mut self, _enabled: bool) {
        SUSPENDED.store(false, Ordering::Release);
    }

    fn suspended(&mut self, suspended: bool) {
        if suspended {
            SUSPENDED.store(true, Ordering::Release);
        } else {
            SUSPENDED.store(false, Ordering::Release);
        }
    }
}
