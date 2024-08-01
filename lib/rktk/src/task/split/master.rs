use embassy_futures::join::join;
use embassy_time::Timer;

use crate::{
    config::MIN_KB_SCAN_INTERVAL,
    constant::LAYER_COUNT,
    interface::{
        backlight::{BacklightCtrl, BacklightMode},
        keyscan::{KeyChangeEventOneHand, KeyscanDriver},
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster},
        usb::{HidReport, UsbDriver},
    },
    keycode::Layer,
    state::{State, StateReport},
    task::backlight::BACKLIGHT_CTRL,
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

pub async fn start<KS: KeyscanDriver, M: MouseDriver, USB: UsbDriver>(
    m2s_tx: M2sTx<'_>,
    s2m_rx: S2mRx<'_>,
    keymap: [Layer; LAYER_COUNT],
    mut key_scanner: KS,
    mut mouse: Option<M>,
    mut usb: USB,
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
            let _ = usb.send_report(HidReport::Keyboard(report)).await;
        }
        if let Some(report) = state_report.mouse_report {
            crate::utils::display_state!(MouseMove, (report.x, report.y));
            let _ = usb.send_report(HidReport::Mouse(report)).await;
        }
        if let Some(report) = state_report.media_keyboard_report {
            let _ = usb.send_report(HidReport::MediaKeyboard(report)).await;
        }

        handle_led(&state_report, m2s_tx, &mut latest_led);

        let took = start.elapsed();
        if took < MIN_KB_SCAN_INTERVAL {
            Timer::after(MIN_KB_SCAN_INTERVAL - took).await;
        }
    }
}
