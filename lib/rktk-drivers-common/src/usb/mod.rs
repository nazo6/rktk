//! Common usb module

use core::sync::atomic::AtomicBool;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

mod builder;
mod driver;
mod handler;
mod rrp;
mod task;

type RemoteWakeupSignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, ()>;
type ReadySignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, ()>;
static SUSPENDED: AtomicBool = AtomicBool::new(false);

pub use builder::CommonUsbDriverBuilder;
pub use embassy_usb::Config;

pub struct UsbOpts<D: embassy_usb::driver::Driver<'static>> {
    pub config: Config<'static>,
    pub mouse_poll_interval: u8,
    pub kb_poll_interval: u8,
    pub driver: D,
}
