use embassy_time::Timer;

use crate::{config::Config, drivers::interface::mouse::MouseDriver};

use super::MOUSE_EVENT_REPORT_CHANNEL;

pub async fn start(mut mouse: Option<impl MouseDriver>, config: &'static Config) {
    if let Some(mouse) = &mut mouse {
        loop {
            Timer::after(config.rktk.scan_interval_mouse).await;

            let mouse_move = match mouse.read().await {
                Ok(m) => m,
                Err(e) => {
                    log::warn!("Failed to read mouse: {:?}", e);
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
