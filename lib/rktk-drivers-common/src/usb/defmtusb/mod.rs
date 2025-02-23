//! Code in this directory is from https://github.com/micro-rust/defmtusb

// FIXME:
#![allow(static_mut_refs)]
#![allow(unused)]

mod buffer;
mod controller;
mod task;

use core::borrow::BorrowMut as _;

use embassy_sync::blocking_mutex::raw::RawMutex as _;
pub use task::logger;

/// The restore state of the critical section.
static mut RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();

/// Indicates if the logger is already taken to avoid reentries.
static mut TAKEN: bool = false;

/// The `defmt` encoder.
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

static QUEUE: rktk::utils::Mutex<heapless::Vec<u8, 1024>> =
    rktk::utils::Mutex::new(heapless::Vec::new());
static LOG_SIGNAL: rktk::utils::Signal<()> = rktk::utils::Signal::new();

/// The logger implementation.
#[defmt::global_logger]
pub struct USBLogger;

unsafe impl defmt::Logger for USBLogger {
    fn acquire() {
        unsafe {
            let restore = critical_section::acquire();
            if TAKEN {
                defmt::error!("defmt logger taken reentrantly");
                defmt::panic!();
            }
            TAKEN = true;
            RESTORE = restore;
            ENCODER.start_frame(inner);
        }
    }

    unsafe fn release() {
        ENCODER.end_frame(inner);
        TAKEN = false;
        let restore = RESTORE;
        critical_section::release(restore);
    }

    unsafe fn flush() {}

    unsafe fn write(bytes: &[u8]) {
        ENCODER.write(bytes, inner);
    }
}

fn inner(bytes: &[u8]) {
    let mut q = QUEUE.try_lock().unwrap();
    if q.capacity() - q.len() < bytes.len() {
        return;
    }

    q.extend_from_slice(bytes);

    LOG_SIGNAL.signal(());
}
