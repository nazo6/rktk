//! USB reporter implementation using [`embassy_usb`].

use core::sync::atomic::AtomicBool;

mod builder;
#[cfg(feature = "defmt-usb")]
mod defmt_logger;
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
/// Re-export of underlying embassy-usb driver's config type
pub use embassy_usb::Config as UsbDriverConfig;

/// Options for the [`CommonUsbDriverBuilder`].
pub struct CommonUsbDriverConfig<D: embassy_usb::driver::Driver<'static>> {
    /// embassy-usb driver instance.
    pub driver: D,
    /// Config for underlying embassy-usb driver.
    pub driver_config: UsbDriverConfig<'static>,
    /// USB Poll interval for mouse in ms.
    pub mouse_poll_interval: u8,
    /// USB Poll interval for keyboard in ms.
    pub keyboard_poll_interval: u8,
    /// If this is set to true, defmt-usb logger waits for DTR signal before log output.
    /// This allows you to view logs recorded before the logger client is started.
    #[cfg(feature = "defmt-usb")]
    pub defmt_usb_use_dtr: bool,
}

impl<D: embassy_usb::driver::Driver<'static>> CommonUsbDriverConfig<D> {
    /// Create usb options for the driver with default options.
    ///
    /// * `driver`: embassy-usb driver instance
    /// * `vid`: USB vendor ID
    /// * `pid`: USB product ID
    pub fn new(driver: D, mut driver_config: UsbDriverConfig<'static>) -> Self {
        driver_config.supports_remote_wakeup = cfg!(feature = "usb-remote-wakeup");
        Self {
            driver_config,
            mouse_poll_interval: 1,
            keyboard_poll_interval: 1,
            driver,
            #[cfg(feature = "defmt-usb")]
            defmt_usb_use_dtr: true,
        }
    }
}
