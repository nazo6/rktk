use embassy_futures::join::join;
use embassy_time::Timer;

use crate::{
    config::{LAYER_COUNT, LEFT_LED_COUNT, SCAN_INTERVAL_KEYBOARD},
    interface::{
        backlight::BacklightDriver,
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        usb::{HidReport, UsbDriver},
    },
    keycode::Layer,
    state::State,
};

pub async fn start<KS: KeyscanDriver, M: MouseDriver, USB: UsbDriver, BL: BacklightDriver>(
    keymap: [Layer; LAYER_COUNT],
    mut key_scanner: KS,
    mut mouse: Option<M>,
    mut usb: USB,
    backlight: Option<BL>,
) {
    join(
        async move {
            if let Some(backlight) = backlight {
                // TODO: Use non-split keyboard specific value
                super::backlight::start::<LEFT_LED_COUNT>(backlight).await;
            }
        },
        async move {
            let mut state = State::new(keymap, None);

            crate::print!("Start",);

            let hand = key_scanner.current_hand().await;

            crate::print!("{:?}", hand);

            loop {
                let start = embassy_time::Instant::now();

                let mut mouse_move: (i8, i8) = (0, 0);

                let (mut master_events, _) = join(key_scanner.scan(), async {
                    if let Some(mouse) = &mut mouse {
                        if let Ok((x, y)) = mouse.read().await {
                            mouse_move.0 += x;
                            mouse_move.1 += y;
                        }
                    }
                })
                .await;

                let state_report = state.update(&mut master_events, &mut [], mouse_move);

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

                let took = start.elapsed();
                if took < SCAN_INTERVAL_KEYBOARD {
                    Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
                }
            }
        },
    )
    .await;
}
