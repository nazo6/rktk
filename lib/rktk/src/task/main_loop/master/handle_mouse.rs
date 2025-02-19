use embassy_time::Timer;

use crate::{config::constant::SCAN_INTERVAL_MOUSE, drivers::interface::mouse::MouseDriver};

use super::MOUSE_EVENT_REPORT_CHANNEL;

pub async fn start(mut mouse: Option<impl MouseDriver>) {
    if let Some(mouse) = &mut mouse {
        loop {
            Timer::after(SCAN_INTERVAL_MOUSE).await;

            let mouse_move = match mouse.read().await {
                Ok(m) => m,
                Err(e) => {
                    rktk_log::warn!(
                        "Failed to read mouse: {:?}",
                        rktk_log::helper::Debug2Format(&e)
                    );
                    crate::print!("{:?}", e);
                    continue;
                }
            };

            if mouse_move == (0, 0) {
                continue;
            } else {
                let _ = MOUSE_EVENT_REPORT_CHANNEL.try_send(mouse_move);
            }
        }
    }
}
