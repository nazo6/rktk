use embassy_time::{Duration, Timer};

use crate::{
    config::DOUBLE_TAP_THRESHOLD,
    constant::LAYER_COUNT,
    interface::{
        double_tap::DoubleTapReset,
        keyscan::Keyscan,
        mouse::Mouse,
        usb::{HidReport, Usb},
    },
    keycode::Layer,
    state::State,
};

pub const MIN_KB_SCAN_INTERVAL: Duration = Duration::from_millis(5);

pub async fn start(
    mut double_tap_reset: Option<impl DoubleTapReset>,
    mut key_scanner: impl Keyscan,
    mut mouse: Option<impl Mouse>,
    mut usb: impl Usb,
    keymap: [Layer; LAYER_COUNT],
) {
    if let Some(dtr) = &mut double_tap_reset {
        dtr.execute(DOUBLE_TAP_THRESHOLD).await;
    }

    let mut state = State::new(keymap, crate::interface::keyscan::Hand::Right);
    loop {
        let start = embassy_time::Instant::now();

        let mut master_events = key_scanner.scan().await;

        let state_report = state.update(&mut master_events, &mut [], (0, 0));

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
            Timer::after(MIN_KB_SCAN_INTERVAL - took).await;
        }
    }
}
