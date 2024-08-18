use embassy_futures::{join::join, select::select};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::{Duration, Timer};
use rktk_keymanager::state::{KeyChangeEvent, State, StateConfig, StateReport};

use crate::{
    config::static_config::{CONFIG, SCAN_INTERVAL_KEYBOARD},
    interface::{
        backlight::{BacklightCtrl, BacklightMode},
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        reporter::ReporterDriver,
        split::{MasterToSlave, SlaveToMaster},
    },
    task::backlight::BACKLIGHT_CTRL,
    Keymap,
};

use super::{M2sTx, S2mRx};

mod rrp_server;

#[macro_export]
macro_rules! get_req {
    ($ep_name:ident, $reporter:expr) => {{
        use rktk_rrp::endpoints::$ep_name::*;
        let mut buf = [0u8; Request::POSTCARD_MAX_SIZE + Request::POSTCARD_MAX_SIZE / 254 + 2];
        read_until_zero($reporter, &mut buf).await;
        let Ok(req) = postcard::from_bytes_cobs::<Request>(&mut buf) else {
            continue;
        };
        req
    }};
}

#[macro_export]
macro_rules! send_res {
    ($ep_name:ident, $reporter:expr, $val:expr) => {{
        use rktk_rrp::endpoints::$ep_name::*;
        let mut buf = [0u8; Response::POSTCARD_MAX_SIZE + Response::POSTCARD_MAX_SIZE / 254 + 2];
        let val: &Response = $val;
        let Ok(res) = postcard::to_slice_cobs(val, &mut buf) else {
            continue;
        };
        let _ = $reporter.send_rrp_data(res).await;
    }};
}

fn split_to_entire(ev: &mut KeyChangeEvent, hand: Hand) {
    if hand == Hand::Right {
        ev.col = CONFIG.cols as u8 - 1 - ev.col;
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

pub async fn start<KS: KeyscanDriver, M: MouseDriver, R: ReporterDriver>(
    m2s_tx: M2sTx<'_>,
    s2m_rx: S2mRx<'_>,
    reporter: &R,
    mut key_scanner: KS,
    mut mouse: Option<M>,
    keymap: Keymap,
    hand: crate::interface::keyscan::Hand,
) {
    let state = Mutex::<ThreadModeRawMutex, _>::new(State::new(
        keymap.clone(),
        StateConfig {
            tap_threshold: Duration::from_millis(CONFIG.default_tap_threshold),
            auto_mouse_layer: CONFIG.default_auto_mouse_layer,
            auto_mouse_duration: Duration::from_millis(CONFIG.default_auto_mouse_duration),
            auto_mouse_threshold: CONFIG.default_auto_mouse_threshold,
            scroll_divider_x: CONFIG.default_scroll_divider_x,
            scroll_divider_y: CONFIG.default_scroll_divider_y,
        },
    ));

    crate::print!("Master start");

    let mut latest_led: Option<BacklightCtrl> = None;

    loop {
        select(
            async {
                loop {
                    let start = embassy_time::Instant::now();

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

                    let state_report = state.lock().await.update(&mut events, mouse_move, start);

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

                    handle_led(&state_report, m2s_tx, &mut latest_led);

                    let took = start.elapsed();

                    if took < SCAN_INTERVAL_KEYBOARD {
                        Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
                    }
                }
            },
            async {
                let mut server = rrp_server::Server { state: &state };
                let et = rrp_server::EndpointTransportImpl(reporter);
                server.handle(&et).await;
            },
        )
        .await;
    }
}
