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
        usb::{HidReport, UsbDriver},
        DriverBuilder,
    },
    keycode::Layer,
    state::State,
};

use self::display::DISPLAY_CONTROLLER;

pub mod display;

pub const MIN_KB_SCAN_INTERVAL: Duration = Duration::from_millis(5);

pub struct Drivers<DTR: DoubleTapReset, KS: Keyscan, M: Mouse, USB: UsbDriver, D: Display> {
    pub key_scanner: KS,
    pub double_tap_reset: Option<DTR>,
    pub mouse: Option<M>,
    pub usb: USB,
    pub display: Option<D>,
}

// Start main task.
// This task does all the processing for rktk.
//
// NOTE: For optimal boot time and proper operation of the Double Tap driver, do not do any heavy processing before executing this function.
// Driver authors should use the init method defined in each driver's trace to perform initialization
// instead of the associated functions such as new that are performed on the keyboard crate side.
pub async fn start<DTR: DoubleTapReset, KS: Keyscan, M: Mouse, USB: UsbDriver, D: Display>(
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
            let usb = &mut drivers.usb;
            let mouse = &mut drivers.mouse;

            if let Some(mouse) = mouse {
                mouse.init().await;
                mouse.set_cpi(600).await;
            }

            let mut state = State::new(keymap, crate::interface::keyscan::Hand::Right);

            DISPLAY_CONTROLLER.signal(display::DisplayMessage::Message("Start"));
            loop {
                let start = embassy_time::Instant::now();

                let mut master_events = drivers.key_scanner.scan().await;

                let mouse_event = if let Some(mouse) = mouse {
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
                    use core::fmt::Write;

                    let mut str = heapless::String::<64>::new();
                    write!(str, "{took}").unwrap();
                    DISPLAY_CONTROLLER.signal(display::DisplayMessage::DynamicMessage(str));
                    Timer::after(MIN_KB_SCAN_INTERVAL - took).await;
                }
            }
        },
    )
    .await;
}
