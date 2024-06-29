//! Common usb module

use core::sync::atomic::AtomicBool;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

pub mod driver;
mod handler;
pub mod interface;

pub static SUSPENDED: AtomicBool = AtomicBool::new(false);
pub type RemoteWakeupSignal = embassy_sync::signal::Signal<CriticalSectionRawMutex, ()>;
