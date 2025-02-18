//! Main task that runs the USB transport layer.

#![allow(unused_labels)]

use embassy_usb::{class::cdc_acm::Sender, driver::Driver};

/// Statically allocated device descriptor buffer.
#[link_section = ".bss.defmtusb.DESCRIPTORS"]
static mut DEVBUF: [u8; 256] = [0u8; 256];

/// Statically allocated configuration descriptor buffer.
#[link_section = ".bss.defmtusb.DESCRIPTORS"]
static mut CFGBUF: [u8; 256] = [0u8; 256];

/// Statically allocated BOS descriptor buffer.
#[link_section = ".bss.defmtusb.DESCRIPTORS"]
static mut BOSBUF: [u8; 256] = [0u8; 256];

/// Statically allocated control buffer.
#[link_section = ".bss.defmtusb.DESCRIPTORS"]
static mut CONTROL: [u8; 256] = [0u8; 256];

/// Runs the logger task.
pub async fn logger<'d, D: Driver<'d>>(mut sender: Sender<'d, D>, size: usize) {
    use embassy_time::{Duration, Timer};

    use embassy_usb::driver::EndpointError;

    // Get a reference to the controller.
    let controller = unsafe { &mut super::controller::CONTROLLER };

    // Get a reference to the buffers.
    let buffers = unsafe { &mut super::controller::BUFFERS };

    'main: loop {
        // Wait for the device to be connected.
        sender.wait_connection().await;

        // Set the controller as enabled.
        controller.enable();

        // Begin sending the data.
        'data: loop {
            // Wait for new data.
            let buffer = 'select: loop {
                // Check which buffer is flushing.
                if buffers[0].flushing() {
                    break 'select &mut buffers[0];
                }
                if buffers[1].flushing() {
                    break 'select &mut buffers[1];
                }

                // Wait the timeout.
                // TODO : Make this configurable.
                Timer::after(Duration::from_millis(100)).await;
            };

            // Get an iterator over the data of the buffer.
            let chunks = buffer.data[..buffer.cursor].chunks(size);

            for chunk in chunks {
                // Send the data.
                if let Err(EndpointError::Disabled) = sender.write_packet(chunk).await {
                    controller.disable();

                    continue 'main;
                }
            }

            // Reset the buffer as it has been transmitted.
            buffer.reset();
        }
    }
}
