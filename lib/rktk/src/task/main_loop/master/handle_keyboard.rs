use embassy_time::Timer;

use crate::{
    config::static_config::SCAN_INTERVAL_KEYBOARD,
    drivers::interface::{
        debounce::DebounceDriver,
        keyscan::{Hand, KeyscanDriver},
    },
};

use super::{utils::resolve_entire_key_pos, KEYBOARD_EVENT_REPORT_CHANNEL};

pub async fn start(
    hand: Hand,
    mut keyscan: impl KeyscanDriver,
    mut debounce: Option<impl DebounceDriver>,
) {
    loop {
        Timer::after(SCAN_INTERVAL_KEYBOARD).await;

        let mut buf = heapless::Vec::<_, 32>::new();
        keyscan
            .scan(|event| {
                let _ = buf.push(event);
            })
            .await;
        for mut event in buf {
            if let Some(debounce) = &mut debounce {
                if debounce.should_ignore_event(&event, embassy_time::Instant::now()) {
                    continue;
                }
            }
            resolve_entire_key_pos(&mut event, hand);

            let _ = KEYBOARD_EVENT_REPORT_CHANNEL.try_send(event);
        }
    }
}
