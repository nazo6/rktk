use embassy_usb::Config;

pub use embassy_usb::Config as UsbConfig;

pub struct UsbUserOpts<'a> {
    pub config: Config<'a>,
    // poll interval
    pub mouse_poll_interval: u8,
    pub kb_poll_interval: u8,
}
