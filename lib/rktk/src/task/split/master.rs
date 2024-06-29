use embassy_futures::join::join;
use embassy_time::Timer;

use crate::{
    constant::LAYER_COUNT,
    interface::{
        keyscan::{KeyChangeEventOneHand, Keyscan},
        mouse::Mouse,
        split::SlaveToMaster,
        usb::{HidReport, UsbDriver},
    },
    keycode::Layer,
    state::State,
    task::{
        display::{DisplayMessage, DISPLAY_CONTROLLER},
        MIN_KB_SCAN_INTERVAL,
    },
};

use super::{M2sTx, S2mRx};

pub async fn start<KS: Keyscan, M: Mouse, USB: UsbDriver>(
    m2s_tx: M2sTx<'_>,
    s2m_rx: S2mRx<'_>,
    keymap: [Layer; LAYER_COUNT],
    mut key_scanner: KS,
    mut mouse: Option<M>,
    mut usb: USB,
) {
    let mut state = State::new(keymap, crate::interface::keyscan::Hand::Right);

    DISPLAY_CONTROLLER.signal(DisplayMessage::Message("Master Start"));

    let mut slave_events = heapless::Vec::<_, 16>::new();

    loop {
        let start = embassy_time::Instant::now();

        slave_events.clear();

        let mut mouse_move: (i8, i8) = (0, 0);

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

        if let Some(report) = state_report.keyboard_report {
            let _ = usb.send_report(HidReport::Keyboard(report)).await;
        }
        if let Some(report) = state_report.mouse_report {
            let _ = usb.send_report(HidReport::Mouse(report)).await;
        }
        if let Some(report) = state_report.media_keyboard_report {
            let _ = usb.send_report(HidReport::MediaKeyboard(report)).await;
        }

        let took = start.elapsed();
        if took < MIN_KB_SCAN_INTERVAL {
            // use core::fmt::Write;
            //
            // let mut str = heapless::String::<64>::new();
            // write!(str, "{took}").unwrap();
            // DISPLAY_CONTROLLER.signal(DisplayMessage::DynamicMessage(str));
            Timer::after(MIN_KB_SCAN_INTERVAL - took).await;
        }
    }
}
