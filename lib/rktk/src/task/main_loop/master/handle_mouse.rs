use embassy_time::Timer;
use rktk_log::warn;

use crate::{
    config::constant::{RKTK_CONFIG, SCAN_INTERVAL_MOUSE},
    drivers::interface::mouse::MouseDriver,
};

use super::MOUSE_EVENT_REPORT_CHANNEL;

pub async fn start(mut mouse: Option<impl MouseDriver>) {
    if let Some(mouse) = &mut mouse {
        loop {
            Timer::after(SCAN_INTERVAL_MOUSE).await;

            let mouse_move = match mouse.read().await {
                Ok(m) => {
                    if RKTK_CONFIG.swap_mouse_x_y {
                        (m.1, m.0)
                    } else {
                        m
                    }
                }
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
            } else if MOUSE_EVENT_REPORT_CHANNEL.try_send(mouse_move).is_err() {
                warn!("Mouse full");
            }
        }
    }
}
