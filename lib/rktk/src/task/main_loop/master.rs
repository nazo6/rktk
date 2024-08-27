use ekv::flash::Flash;
use embassy_futures::{join::join, select::select};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::Timer;
use rktk_keymanager::state::{
    config::{KeyResolverConfig, MouseConfig, StateConfig},
    KeyChangeEvent, State, StateReport,
};

use crate::{
    config::flash_config::ReadConfig as _,
    config::static_config::{CONFIG, SCAN_INTERVAL_KEYBOARD},
    interface::{
        backlight::{BacklightCtrl, BacklightMode},
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        reporter::ReporterDriver,
        split::{MasterToSlave, SlaveToMaster},
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
pub async fn start<'a, KS: KeyscanDriver, M: MouseDriver, R: ReporterDriver, EkvFlash: Flash>(
    m2s_tx: M2sTx<'a>,
    s2m_rx: S2mRx<'a>,
    reporter: &'a R,
    mut key_scanner: KS,
    storage: Option<&'a ekv::Database<EkvFlash, CriticalSectionRawMutex>>,
    mut mouse: Option<M>,
    key_config: KeyConfig,
    hand: crate::interface::keyscan::Hand,
) {
    let now = embassy_time::Instant::now();
    let (state_config, keymap) = if let Some(storage) = storage {
        if (storage.mount().await).is_err() {
            if let Err(e) = storage.format().await {
                crate::print!("Failed to format storage: {:?}", e);
            } else {
                crate::print!("Storage formatted");
            }
        }

        let mut tx = storage.write_transaction().await;
        tx.write(&[0, 1, 2, 3], &[4, 5, 6, 7]).await.unwrap();
        tx.commit().await.unwrap();

        let mut buf = [0, 0, 0, 0];
        storage
            .read_transaction()
            .await
            .read(&[0, 1, 2, 3], &mut buf)
            .await
            .expect("read fail");
        assert_eq!(&buf, &[4, 5, 6, 7]);

        let tx = storage.read_transaction().await;

        let mut keymap = key_config.keymap;
        for l in 0..CONFIG.layer_count {
            match tx.read_keymap(l as u32).await {
                Ok(layer) => {
                    keymap[l as usize] = layer;
                }
                Err(e) => {
                    // crate::print!("read:{:?}", e);
                }
            }
        }

        let c = tx.read_state_config().await;

        (c.ok(), keymap)
    } else {
        (None, key_config.keymap)
    };

    crate::print!("config load: {}ms", now.elapsed().as_millis());

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

    loop {
        select(
            async {
                let mut prev_time = embassy_time::Instant::now();
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

                    let state_report = state.lock().await.update(
                        &mut events,
                        mouse_move,
                        since_last_update.into(),
                    );

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
                        if let Some(storage) = storage {
                            let _ = storage.format().await;
                            crate::print!("Storage formatted");
                        }
                    }

                    handle_led(&state_report, m2s_tx, &mut latest_led);

                    let took = start.elapsed();

                    if took < SCAN_INTERVAL_KEYBOARD {
                        Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
                    }
                }
            },
            async {
                let mut server = rrp_server::Server {
                    state: &state,
                    storage,
                };
                let et = rrp_server::EndpointTransportImpl(reporter);
                server.handle(&et).await;
            },
        )
        .await;
    }
}
