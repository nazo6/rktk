use rktk_log::{debug, warn};
use embassy_futures::select::{select, Either};

use super::utils::get_split_right_shift;
use crate::{
    config::Hand,
    config::schema::DynamicConfig,
    drivers::interface::{debounce::DebounceDriver, keyscan::KeyscanDriver},
    task::channels::report::{KEYBOARD_EVENT_REPORT_CHANNEL, KEYBOARD_CONTROL_CHANNEL, KeyboardCommand},
};

use super::utils::resolve_entire_key_pos;

pub async fn start(
    config: &'static DynamicConfig,
    hand: Hand,
    mut keyscan: impl KeyscanDriver,
    debounce: &mut Option<impl DebounceDriver>,
) {
    debug!("keyscan start");
    let mut latest = embassy_time::Instant::from_millis(0);
    let interval = embassy_time::Duration::from_millis(config.rktk.scan_interval_keyboard);
    let shift = get_split_right_shift(config);
    loop {
        match select(
            KEYBOARD_CONTROL_CHANNEL.receive(),
            async {
                let elapsed = latest.elapsed();
                if elapsed < interval {
                    embassy_time::Timer::after(interval - elapsed).await;
                }

                keyscan
                    .scan(|mut event| {
                        if let Some(debounce) = debounce.as_mut()
                            && debounce.should_ignore_event(&event, embassy_time::Instant::now())
                        {
                            debug!("Debounced");
                            return;
                        }
                        resolve_entire_key_pos(&mut event, hand, shift);

                        if KEYBOARD_EVENT_REPORT_CHANNEL.try_send(event).is_err() {
                            warn!("Keyboard full");
                        }
                    })
                    .await;
                latest = embassy_time::Instant::now();
            }
        ).await {
            Either::First(cmd) => {
                match cmd {
                    KeyboardCommand::StartCalibration => {
                        debug!("Starting calibration");
                        keyscan.start_calibration();
                    }
                    KeyboardCommand::EndCalibration => {
                        debug!("Ending calibration");
                        keyscan.end_calibration();
                        // TODO: handle saving calibration data
                    }
                }
            }
            Either::Second(_) => {}
        }
    }
}
