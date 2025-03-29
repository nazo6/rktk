//! Main task that runs the USB transport layer.

use embassy_usb::{
    class::cdc_acm::Sender,
    driver::{Driver, EndpointError},
};

use super::{LOG_SIGNAL, QUEUE};

/// Runs the logger task.
pub async fn logger<'d, D: Driver<'d>>(mut sender: Sender<'d, D>, size: usize, use_dtr: bool) {
    sender.wait_connection().await;

    loop {
        if use_dtr {
            loop {
                if sender.dtr() {
                    break;
                }
                embassy_time::Timer::after_millis(500).await;
            }
        }

        LOG_SIGNAL.wait().await;
        embassy_time::Timer::after_millis(100).await;
        LOG_SIGNAL.reset();

        let mut q = QUEUE.lock().await;

        for chunk in q.chunks(size) {
            if let Err(EndpointError::Disabled) = sender.write_packet(chunk).await {
                sender.wait_connection().await;
            }
        }

        q.clear();
    }
}
