use embassy_time::Timer;

use crate::{config::static_config::SCAN_INTERVAL_MOUSE, drivers::interface::mouse::MouseDriver};

use super::MOUSE_EVENT_REPORT_CHANNEL;

pub async fn start(mut mouse: Option<impl MouseDriver>) {
    if let Some(mouse) = &mut mouse {
        let mut empty_sent = false;
        loop {
            Timer::after(SCAN_INTERVAL_MOUSE).await;

            let mouse_move = match mouse.read().await {
                Ok(m) => m,
                Err(e) => {
                    log::warn!("Failed to read mouse: {:?}", e);
                    crate::print!("{:?}", e);
                    continue;
                }
            };

            if mouse_move == (0, 0) && empty_sent {
                continue;
            } else {
                let _ = MOUSE_EVENT_REPORT_CHANNEL.try_send(mouse_move);
                empty_sent = mouse_move == (0, 0);
            }
        }
    }
}