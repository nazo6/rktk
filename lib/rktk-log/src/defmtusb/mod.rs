//! Code in this directory is from https://github.com/micro-rust/defmtusb

// FIXME:
#![allow(static_mut_refs)]
#![allow(unused)]

mod buffer;
mod controller;
mod task;

pub use task::logger;

/// The restore state of the critical section.
#[link_section = ".bss.defmt-usb.RESTORE"]
static mut RESTORE: critical_section::RestoreState = critical_section::RestoreState::invalid();

/// Indicates if the logger is already taken to avoid reentries.
#[link_section = ".bss.defmt-usb.TAKEN"]
static mut TAKEN: bool = false;

/// The `defmt` encoder.
#[link_section = ".data.defmt-usb"]
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

/// The logger implementation.
#[defmt::global_logger]
pub struct USBLogger;

unsafe impl defmt::Logger for USBLogger {
    fn acquire() {
        unsafe {
            // Get in a critical section.
            let restore = critical_section::acquire();

            // Check for reentries.
            if TAKEN {
                defmt::error!("defmt logger taken reentrantly");
                defmt::panic!();
            }

            // Set the taken flag.
            TAKEN = true;

            // Save the restore state.
            RESTORE = restore;

            // Start the frame.
            ENCODER.start_frame(inner);
        }
    }

    unsafe fn release() {
        // End the current frame.
        ENCODER.end_frame(inner);

        // Restore the token.
        TAKEN = false;

        // Get the restore state of the critical section.
        let restore = RESTORE;

        controller::CONTROLLER.swap();

        // Restore the critical section.
        critical_section::release(restore);
    }

    unsafe fn flush() {
        controller::CONTROLLER.swap()
    }

    unsafe fn write(bytes: &[u8]) {
        ENCODER.write(bytes, inner);
    }
}

fn inner(bytes: &[u8]) {
    // Get a reference to the buffers.
    let controller = unsafe { &mut controller::CONTROLLER };

    // Write to the next buffer.
    controller.write(bytes);
}
