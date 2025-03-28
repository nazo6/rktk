//! Common usb driver implementation over [`embassy_usb`].

use core::sync::atomic::AtomicBool;

mod builder;
#[cfg(feature = "defmtusb")]
mod defmtusb;
mod driver;
mod handler;
mod raw_hid;
mod rrp;
mod task;

#[cfg(feature = "usb-remote-wakeup")]
type RemoteWakeupSignal = rktk::utils::Signal<()>;
type ReadySignal = rktk::utils::Signal<()>;
static SUSPENDED: AtomicBool = AtomicBool::new(false);

pub use builder::CommonUsbDriverBuilder;
pub use embassy_usb::Config as UsbDriverConfig;

pub struct UsbOpts<D: embassy_usb::driver::Driver<'static>> {
    pub config: UsbDriverConfig<'static>,
    pub mouse_poll_interval: u8,
    pub kb_poll_interval: u8,
    pub driver: D,
    #[cfg(feature = "defmtusb")]
    pub defmt_usb_use_dtr: bool,
}
