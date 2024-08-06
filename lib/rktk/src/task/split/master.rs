use embassy_futures::join::join;
use embassy_time::Timer;

use crate::{
    config::static_config::{CONFIG, SCAN_INTERVAL_KEYBOARD},
    interface::{
        backlight::{BacklightCtrl, BacklightMode},
        keyscan::{KeyChangeEventOneHand, KeyscanDriver},
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster},
        usb::HidReport,
    },
    keycode::Layer,
    state::{State, StateReport},
    task::{backlight::BACKLIGHT_CTRL, report::ReportSender},
};

use super::{M2sTx, S2mRx};

fn receive_from_slave(
    slave_events: &mut heapless::Vec<KeyChangeEventOneHand, 16>,
    mouse_move: &mut (i8, i8),
    s2m_rx: S2mRx<'_>,
) {
    slave_events.clear();
    while let Ok(cmd_from_slave) = s2m_rx.try_receive() {
        match cmd_from_slave {
            SlaveToMaster::Pressed(row, col) => {
                slave_events
                    .push(KeyChangeEventOneHand {
                        col,
                        row,
                        pressed: true,
                    })
                    .ok();
            }
            SlaveToMaster::Released(row, col) => {
                slave_events
                    .push(KeyChangeEventOneHand {
                        col,
                        row,
                        pressed: false,
                    })
                    .ok();
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

pub async fn start<KS: KeyscanDriver, M: MouseDriver>(
    m2s_tx: M2sTx<'_>,
    s2m_rx: S2mRx<'_>,
    report_sender: ReportSender<'_>,
    mut key_scanner: KS,
    mut mouse: Option<M>,
    keymap: [Layer; CONFIG.layer_count],
    hand: crate::interface::keyscan::Hand,
) {
    let mut state = State::new(keymap, Some(hand));

    crate::print!("Master start");

    let mut latest_led: Option<BacklightCtrl> = None;

    let mut slave_events = heapless::Vec::<_, 16>::new();

    loop {
        let start = embassy_time::Instant::now();

        let mut mouse_move: (i8, i8) = (0, 0);

        receive_from_slave(&mut slave_events, &mut mouse_move, s2m_rx);

        let (mut master_events, _) = join(key_scanner.scan(), async {
            if let Some(mouse) = &mut mouse {
                if let Ok((x, y)) = mouse.read().await {
                    mouse_move.0 += x;
                    mouse_move.1 += y;
                }
            }
        })
        .await;

        let state_report = state.update(&mut master_events, &mut slave_events, mouse_move);

        crate::utils::display_state!(HighestLayer, state_report.highest_layer);

        if let Some(report) = state_report.keyboard_report {
            let _ = report_sender.try_send(HidReport::Keyboard(report));
        }
        if let Some(report) = state_report.mouse_report {
            crate::utils::display_state!(MouseMove, (report.x, report.y));
            let _ = report_sender.try_send(HidReport::Mouse(report));
        }
        if let Some(report) = state_report.media_keyboard_report {
            let _ = report_sender.try_send(HidReport::MediaKeyboard(report));
        }

        handle_led(&state_report, m2s_tx, &mut latest_led);

        let took = start.elapsed();

        crate::print!("Master took {:?}", took);
        if took < SCAN_INTERVAL_KEYBOARD {
            Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
        }
    }
}
