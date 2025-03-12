use core::sync::atomic::Ordering;
use embassy_usb::{
    Handler,
    class::hid::{ReportId, RequestHandler},
    control::OutResponse,
};

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

pub struct UsbRequestHandler {}

impl RequestHandler for UsbRequestHandler {
    fn get_report(&mut self, _id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        None
    }

    fn set_report(&mut self, _id: ReportId, _data: &[u8]) -> OutResponse {
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, _id: Option<ReportId>, _dur: u32) {}

    fn get_idle_ms(&mut self, _id: Option<ReportId>) -> Option<u32> {
        None
    }
}
