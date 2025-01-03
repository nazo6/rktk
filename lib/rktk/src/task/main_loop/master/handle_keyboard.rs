use embassy_time::Timer;

use crate::{
    config::Config,
    drivers::interface::{
        debounce::DebounceDriver,
        keyscan::{Hand, KeyscanDriver},
    },
    task::channels::report::KEYBOARD_EVENT_REPORT_CHANNEL,
};

use super::utils::resolve_entire_key_pos;

pub async fn start(
    hand: Hand,
    mut keyscan: impl KeyscanDriver,
    mut debounce: Option<impl DebounceDriver>,
    config: &'static Config,
) {
    loop {
        Timer::after(config.rktk.scan_interval_keyboard).await;

        keyscan
            .scan(|mut event| {
                if let Some(debounce) = &mut debounce {
                    if debounce.should_ignore_event(&event, embassy_time::Instant::now()) {
                        return;
                    }
                }
                resolve_entire_key_pos(&mut event, hand);

                let _ = KEYBOARD_EVENT_REPORT_CHANNEL.try_send(event);
            })
            .await;
    }
}
