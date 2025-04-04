use crate::{
    config::constant::{RKTK_CONFIG, SCAN_INTERVAL_MOUSE},
    drivers::interface::mouse::MouseDriver,
    task::channels::report::update_mouse,
};

pub async fn start(mut mouse: Option<impl MouseDriver>) {
    if let Some(mouse) = &mut mouse {
        let mut latest = embassy_time::Instant::from_millis(0);
        loop {
            let elapsed = latest.elapsed();
            if elapsed < SCAN_INTERVAL_MOUSE {
                embassy_time::Timer::after(SCAN_INTERVAL_MOUSE - elapsed).await;
            }

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
            }

            update_mouse(mouse_move.0, mouse_move.1);

            latest = embassy_time::Instant::now();
        }
    }
}
