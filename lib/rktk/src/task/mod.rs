use embassy_futures::join::join;
use embassy_time::{Duration, Timer};

use crate::{
    config::DOUBLE_TAP_THRESHOLD,
    constant::LAYER_COUNT,
    interface::{
        display::{Display, GlobalDisplay},
        double_tap::DoubleTapReset,
        keyscan::Keyscan,
        mouse::Mouse,
        usb::{HidReport, Usb},
    },
    keycode::Layer,
    state::State,
};

use self::display::DISPLAY_CONTROLLER;

pub mod display;

pub const MIN_KB_SCAN_INTERVAL: Duration = Duration::from_millis(5);

pub struct Drivers<DTR: DoubleTapReset, KS: Keyscan, M: Mouse, USB: Usb, D: Display> {
    pub key_scanner: KS,
    pub double_tap_reset: Option<DTR>,
    pub mouse: Option<M>,
    pub usb: USB,
    pub display: Option<D>,
}

pub async fn start<DTR: DoubleTapReset, KS: Keyscan, M: Mouse, USB: Usb, D: Display>(
    mut drivers: Drivers<DTR, KS, M, USB, D>,
    keymap: [Layer; LAYER_COUNT],
) {
    if let Some(dtr) = &mut drivers.double_tap_reset {
        dtr.execute(DOUBLE_TAP_THRESHOLD).await;
    }

    join(
        async move {
            if let Some(display) = drivers.display {
                display::start(display).await;
            }
        },
        async {
            let mut state = State::new(keymap, crate::interface::keyscan::Hand::Right);
            loop {
                DISPLAY_CONTROLLER.signal(display::DisplayMessage::Message("Start"));
                let start = embassy_time::Instant::now();

                let mut master_events = drivers.key_scanner.scan().await;

                let mouse_event = if let Some(mouse) = &mut drivers.mouse {
                    if let Ok(e) = mouse.read().await {
                        Some(e)
                    } else {
                        None
                    }
                } else {
                    None
                };

                let state_report =
                    state.update(&mut master_events, &mut [], mouse_event.unwrap_or_default());

                if let Some(report) = state_report.keyboard_report {
                    let _ = drivers.usb.send_report(HidReport::Keyboard(report)).await;
                }
                if let Some(report) = state_report.mouse_report {
                    let _ = drivers.usb.send_report(HidReport::Mouse(report)).await;
                }
                if let Some(report) = state_report.media_keyboard_report {
                    let _ = drivers
                        .usb
                        .send_report(HidReport::MediaKeyboard(report))
                        .await;
                }

                let took = start.elapsed();
                if took < MIN_KB_SCAN_INTERVAL {
                    Timer::after(MIN_KB_SCAN_INTERVAL - took).await;
                }
            }
        },
    )
    .await;
}
