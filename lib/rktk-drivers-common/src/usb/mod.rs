//! Common usb module

use core::sync::atomic::AtomicBool;

mod builder;
mod driver;
mod handler;
mod rrp;
mod task;

type RemoteWakeupSignal = rktk::utils::Signal<()>;
type ReadySignal = rktk::utils::Signal<()>;
static SUSPENDED: AtomicBool = AtomicBool::new(false);

pub use builder::CommonUsbDriverBuilder;
pub use embassy_usb::Config;

pub struct UsbOpts<D: embassy_usb::driver::Driver<'static>> {
    pub config: Config<'static>,
    pub mouse_poll_interval: u8,
    pub kb_poll_interval: u8,
    pub driver: D,
}
