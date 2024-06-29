use embassy_usb::class::hid::{RequestHandler, State};
use embassy_usb::driver::Driver;
use embassy_usb::{Config, Handler};

pub use embassy_usb::Config as UsbConfig;

pub struct UsbOpts<'a, D: Driver<'a>> {
    pub kb_request_handler: &'a mut dyn RequestHandler,
    pub mouse_request_handler: &'a mut dyn RequestHandler,
    pub mkb_request_handler: &'a mut dyn RequestHandler,
    pub device_handler: &'a mut dyn Handler,
    pub resource: UsbResource<'a, D>,
}
pub struct UsbUserOpts<'a> {
    pub config: Config<'a>,
    // poll interval
    pub mouse_poll_interval: u8,
    pub kb_poll_interval: u8,
}

pub struct UsbResource<'a, D: Driver<'a>> {
    pub driver: D,
    pub config_descriptor: &'a mut [u8],
    pub bos_descriptor: &'a mut [u8],
    pub msos_descriptor: &'a mut [u8],
    pub control_buf: &'a mut [u8],
    pub state_kb: &'a mut State<'a>,
    pub state_mouse: &'a mut State<'a>,
    pub state_media_key: &'a mut State<'a>,
}
