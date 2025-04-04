use rktk_log::{debug, warn};

use crate::{
    config::constant::SCAN_INTERVAL_KEYBOARD,
    drivers::interface::{debounce::DebounceDriver, keyscan::KeyscanDriver},
    interface::Hand,
    task::channels::report::KEYBOARD_EVENT_REPORT_CHANNEL,
};

use super::utils::resolve_entire_key_pos;

pub async fn start(
    hand: Hand,
    mut keyscan: impl KeyscanDriver,
    debounce: &mut Option<impl DebounceDriver>,
) {
    debug!("keyscan start");
    let mut latest = embassy_time::Instant::from_millis(0);
    loop {
        let elapsed = latest.elapsed();
        if elapsed < SCAN_INTERVAL_KEYBOARD {
            embassy_time::Timer::after(SCAN_INTERVAL_KEYBOARD - elapsed).await;
        }

        keyscan
            .scan(|mut event| {
                if let Some(debounce) = debounce.as_mut() {
                    if debounce.should_ignore_event(&event, embassy_time::Instant::now()) {
                        debug!("Debounced");
                        return;
                    }
                }
                resolve_entire_key_pos(&mut event, hand);

                if KEYBOARD_EVENT_REPORT_CHANNEL.try_send(event).is_err() {
                    warn!("Keyboard full");
                }
            })
            .await;

        latest = embassy_time::Instant::now();
    }
}
