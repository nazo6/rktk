//! Common usb module

use core::sync::atomic::AtomicBool;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

mod builder;
mod driver;
mod handler;

type RemoteWakeupSignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, ()>;
static SUSPENDED: AtomicBool = AtomicBool::new(false);

pub use builder::CommonUsbDriverBuilder;
pub use embassy_usb::Config;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

pub struct UsbOpts<D: embassy_usb::driver::Driver<'static>> {
    pub config: Config<'static>,
    pub mouse_poll_interval: u8,
    pub kb_poll_interval: u8,
    pub driver: D,
}

static HID_KEYBOARD_CHANNEL: Channel<CriticalSectionRawMutex, KeyboardReport, 8> = Channel::new();
static HID_MOUSE_CHANNEL: Channel<CriticalSectionRawMutex, MouseReport, 8> = Channel::new();
static HID_MEDIA_KEYBOARD_CHANNEL: Channel<CriticalSectionRawMutex, MediaKeyboardReport, 8> =
    Channel::new();
