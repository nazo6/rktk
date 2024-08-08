use embassy_futures::join::join;
use embassy_time::{Duration, Timer};
use rktk_keymanager::state::{State, StateConfig};

use crate::{
    config::static_config::{CONFIG, SCAN_INTERVAL_KEYBOARD},
    interface::{
        backlight::BacklightDriver, keyscan::KeyscanDriver, mouse::MouseDriver, usb::HidReport,
    },
    Layer,
};

use super::report::ReportSender;

pub async fn start<KS: KeyscanDriver, M: MouseDriver, BL: BacklightDriver>(
    report_sender: ReportSender<'_>,
    keymap: [Layer; CONFIG.layer_count],
    mut key_scanner: KS,
    mut mouse: Option<M>,
    backlight: Option<BL>,
) {
    join(
        async move {
            if let Some(backlight) = backlight {
                // TODO: Use non-split keyboard specific value
                super::backlight::start::<{ CONFIG.left_led_count }>(backlight).await;
            }
        },
        async move {
            let mut state = State::new(
                keymap,
                StateConfig {
                    tap_threshold: Duration::from_millis(CONFIG.default_tap_threshold),
                    auto_mouse_layer: CONFIG.default_auto_mouse_layer,
                    auto_mouse_duration: Duration::from_millis(CONFIG.default_auto_mouse_duration),
                    auto_mouse_threshold: CONFIG.default_auto_mouse_threshold,
                    scroll_divider_x: CONFIG.default_scroll_divider_x,
                    scroll_divider_y: CONFIG.default_scroll_divider_y,
                },
            );

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

                let state_report = state.update(&mut master_events, mouse_move, start);

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

                let took = start.elapsed();
                if took < SCAN_INTERVAL_KEYBOARD {
                    Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
                }
            }
        },
    )
    .await;
}
