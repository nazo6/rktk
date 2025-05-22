use embassy_time::Duration;

use crate::{
    config::constant::schema::DynamicConfig, drivers::interface::mouse::MouseDriver,
    task::channels::report::update_mouse,
};

pub async fn start(config: &'static DynamicConfig, mut mouse: Option<impl MouseDriver>) {
    if let Some(mouse) = &mut mouse {
        let mut latest = embassy_time::Instant::from_millis(0);
        let interval = Duration::from_millis(config.rktk.scan_interval_mouse);
        loop {
            let elapsed = latest.elapsed();
            if elapsed < interval {
                embassy_time::Timer::after(interval - elapsed).await;
            }

            let mouse_move = match mouse.read().await {
                Ok(m) => {
                    if config.rktk.swap_mouse_x_y {
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
            }

            update_mouse(mouse_move.0, mouse_move.1);

            latest = embassy_time::Instant::now();
        }
    }
}
