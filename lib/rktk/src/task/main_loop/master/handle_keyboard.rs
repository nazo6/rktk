use embassy_time::Timer;
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
    mut debounce: Option<impl DebounceDriver>,
) {
    debug!("keyscan start");
    loop {
        Timer::after(SCAN_INTERVAL_KEYBOARD).await;

        keyscan
            .scan(|mut event| {
                if let Some(debounce) = &mut debounce {
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
    }
}
