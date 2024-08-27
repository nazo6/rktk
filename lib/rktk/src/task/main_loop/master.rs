use embassy_futures::join::join;
use embassy_time::{Duration, Timer};
use rktk_keymanager::state::{
    config::{KeyResolverConfig, MouseConfig, StateConfig},
    KeyChangeEvent, State, StateReport,
};

use crate::{
    config::{
        static_config::{CONFIG, SCAN_INTERVAL_KEYBOARD},
        storage_config::StorageConfigManager,
    },
    interface::{
        backlight::{BacklightCtrl, BacklightMode},
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        reporter::ReporterDriver,
        split::{MasterToSlave, SlaveToMaster},
        storage::StorageDriver,
    },
    task::backlight::BACKLIGHT_CTRL,
    utils::ThreadModeMutex,
    KeyConfig,
};

use super::{M2sTx, S2mRx};

mod rrp_server;

fn split_to_entire(ev: &mut KeyChangeEvent, hand: Hand) {
    if hand == Hand::Right {
        ev.col = CONFIG.cols - 1 - ev.col;
    }
}

fn receive_from_slave<const N: usize>(
    slave_events: &mut heapless::Vec<KeyChangeEvent, N>,
    mouse_move: &mut (i8, i8),
    hand: Hand,
    s2m_rx: S2mRx<'_>,
) {
    while let Ok(cmd_from_slave) = s2m_rx.try_receive() {
        match cmd_from_slave {
            SlaveToMaster::Pressed(row, col) => {
                let mut ev = KeyChangeEvent {
                    col,
                    row,
                    pressed: true,
                };
                split_to_entire(&mut ev, hand);
                slave_events.push(ev).ok();
            }
            SlaveToMaster::Released(row, col) => {
                let mut ev = KeyChangeEvent {
                    col,
                    row,
                    pressed: false,
                };
                split_to_entire(&mut ev, hand);
                slave_events.push(ev).ok();
            }
            SlaveToMaster::Mouse { x, y } => {
                mouse_move.0 += x;
                mouse_move.1 += y;
            }
            SlaveToMaster::Message(_) => {}
        }
    }
}

fn handle_led(
    state_report: &StateReport,
    m2s_tx: M2sTx<'_>,
    latest_led: &mut Option<BacklightCtrl>,
) {
    let led = match state_report.highest_layer {
        1 => BacklightCtrl::Start(BacklightMode::SolidColor(0, 0, 1)),
        2 => BacklightCtrl::Start(BacklightMode::SolidColor(1, 0, 0)),
        3 => BacklightCtrl::Start(BacklightMode::SolidColor(0, 1, 0)),
        4 => BacklightCtrl::Start(BacklightMode::SolidColor(1, 1, 0)),
        _ => BacklightCtrl::Start(BacklightMode::SolidColor(0, 0, 0)),
    };

    if let Some(latest_led) = &latest_led {
        if led != *latest_led {
            let _ = BACKLIGHT_CTRL.try_send(led.clone());
            let _ = m2s_tx.try_send(MasterToSlave::Backlight(led.clone()));
        }
    }

    *latest_led = Some(led);
}

#[allow(clippy::too_many_arguments)]
pub async fn start<'a, KS: KeyscanDriver, M: MouseDriver, R: ReporterDriver, S: StorageDriver>(
    m2s_tx: M2sTx<'a>,
    s2m_rx: S2mRx<'a>,
    reporter: &'a R,
    mut key_scanner: KS,
    storage: Option<S>,
    mut mouse: Option<M>,
    key_config: KeyConfig,
    hand: crate::interface::keyscan::Hand,
) {
    let mut config_storage = None;
    if let Some(s) = storage {
        let s = StorageConfigManager::new(s);

        match s.read_version().await {
            Ok(1) => {
                // crate::print!("Storage version matched!");
                config_storage = Some(s);
            }
            Ok(i) => {
                crate::print!("Storage version mismatch: {}", i);
            }
            Err(_e) => match s.write_version(1).await {
                Ok(_) => {
                    config_storage = Some(s);
                }
                Err(e) => {
                    crate::print!("Failed to access storage: {:?}", e);
                }
            },
        }
    }

    let (state_config, keymap) = if let Some(storage) = &config_storage {
        let mut keymap = key_config.keymap;
        for l in 0..CONFIG.layer_count {
            if let Ok(layer) = storage.read_keymap(l).await {
                keymap[l as usize] = layer;
            }
        }

        let c = storage.read_state_config().await;

        (c.ok(), keymap)
    } else {
        (None, key_config.keymap)
    };

    let state_config = state_config.unwrap_or_else(|| StateConfig {
        mouse: MouseConfig {
            auto_mouse_layer: CONFIG.default_auto_mouse_layer,
            auto_mouse_duration: CONFIG.default_auto_mouse_duration,
            auto_mouse_threshold: CONFIG.default_auto_mouse_threshold,
            scroll_divider_x: CONFIG.default_scroll_divider_x,
            scroll_divider_y: CONFIG.default_scroll_divider_y,
        },
        key_resolver: KeyResolverConfig {
            tap_threshold: CONFIG.default_tap_threshold,
            tap_dance_threshold: CONFIG.default_tap_dance_threshold,
            tap_dance: key_config.tap_dance.clone(),
        },
    });

    let state = ThreadModeMutex::new(State::new(keymap, state_config));

    // crate::print!("Master start");

    let mut latest_led: Option<BacklightCtrl> = None;

    join(
        async {
            let mut prev_time = embassy_time::Instant::now();

            let mut duration_max = Duration::from_millis(0);
            let mut duration_sum = Duration::from_millis(0);
            let mut loop_count = 0;

            loop {
                let start = embassy_time::Instant::now();
                let since_last_update = start - prev_time;
                prev_time = start;

                let mut mouse_move: (i8, i8) = (0, 0);

                let (mut events, _) = join(key_scanner.scan(), async {
                    if let Some(mouse) = &mut mouse {
                        if let Ok((x, y)) = mouse.read().await {
                            mouse_move.0 += x;
                            mouse_move.1 += y;
                        }
                    }
                })
                .await;

                events.iter_mut().for_each(|ev| split_to_entire(ev, hand));

                receive_from_slave(&mut events, &mut mouse_move, hand.other(), s2m_rx);

                let state_report =
                    state
                        .lock()
                        .await
                        .update(&mut events, mouse_move, since_last_update.into());

                crate::utils::display_state!(HighestLayer, state_report.highest_layer);

                if let Some(report) = state_report.keyboard_report {
                    let _ = reporter.try_send_keyboard_report(report);
                    let _ = reporter.wakeup();
                }
                if let Some(report) = state_report.mouse_report {
                    crate::utils::display_state!(MouseMove, (report.x, report.y));
                    let _ = reporter.try_send_mouse_report(report);
                }
                if let Some(report) = state_report.media_keyboard_report {
                    let _ = reporter.try_send_media_keyboard_report(report);
                }
                if state_report.transparent_report.flash_clear {
                    if let Some(ref storage) = config_storage {
                        match storage.storage.format().await {
                            Ok(_) => crate::print!("Storage formatted"),
                            Err(e) => crate::print!("Failed to format storage: {:?}", e),
                        }
                    }
                }

                handle_led(&state_report, m2s_tx, &mut latest_led);

                let took = start.elapsed();

                if took > duration_max {
                    duration_max = took;
                }
                duration_sum += took;
                loop_count += 1;
                if loop_count % 100 == 0 {
                    log::info!(
                        "Max: {}us, Avg: {}us",
                        duration_max.as_micros(),
                        duration_sum.as_micros() / loop_count
                    );
                    duration_max = Duration::from_millis(0);
                    duration_sum = Duration::from_millis(0);
                    loop_count = 0;
                }

                if took < SCAN_INTERVAL_KEYBOARD {
                    Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
                }
            }
        },
        async {
            let mut server = rrp_server::Server {
                state: &state,
                storage: config_storage.as_ref(),
            };
            let et = rrp_server::EndpointTransportImpl(reporter);
            server.handle(&et).await;
        },
    )
    .await;
}
