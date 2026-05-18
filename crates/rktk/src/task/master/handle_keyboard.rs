use embassy_futures::select::{Either, select};
use rktk_log::{debug, helper::Debug2Format, warn};

use super::utils::get_split_right_shift;
use crate::{
    config::Hand,
    config::schema::DynamicConfig,
    config::storage::StorageConfigManager,
    drivers::interface::{
        debounce::DebounceDriver, keyscan::KeyscanDriver, storage::StorageDriver,
    },
    task::channels::report::{
        KEYBOARD_CONTROL_CHANNEL, KEYBOARD_EVENT_REPORT_CHANNEL, KeyboardCommand,
    },
};

use super::utils::resolve_entire_key_pos;

pub async fn start<KeyScan: KeyscanDriver, Debounce: DebounceDriver, S: StorageDriver>(
    config: &'static DynamicConfig,
    hand: Hand,
    mut keyscan: KeyScan,
    debounce: &mut Option<Debounce>,
    config_store: &Option<StorageConfigManager<S>>,
) {
    debug!("keyscan start");
    let mut latest = embassy_time::Instant::from_millis(0);
    let interval = embassy_time::Duration::from_millis(config.rktk.scan_interval_keyboard);
    let shift = get_split_right_shift(config);
    loop {
        match select(KEYBOARD_CONTROL_CHANNEL.receive(), async {
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
        })
        .await
        {
            Either::First(cmd) => match cmd {
                KeyboardCommand::StartCalibration => {
                    debug!("Starting calibration");
                    keyscan.start_calibration();
                }
                KeyboardCommand::EndCalibration => {
                    debug!("Ending calibration");
                    keyscan.end_calibration();

                    if KeyScan::CALIBRATION_SIZE > 0
                        && let Some(store) = config_store.as_ref()
                    {
                        let mut buf = [0u8; crate::config::CONST_CONFIG.buffer.calibration];
                        if keyscan.save_calibration(&mut buf[..KeyScan::CALIBRATION_SIZE]).is_ok() {
                            match store.write_calibration::<{ crate::config::CONST_CONFIG.buffer.calibration }>(&buf).await {
                                        Ok(()) => {
                                            debug!("Calibration data saved to flash successfully");
                                        }
                                        Err(e) => {
                                            warn!("Failed to save calibration data: {:?}", Debug2Format(&e));
                                        }
                                    }
                        }
                    }
                }
            },
            Either::Second(_) => {}
        }
    }
}
