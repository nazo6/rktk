use embassy_futures::select::{Either, select};
use embassy_time::Duration;
use rktk_log::debug;

use crate::{
    drivers::interface::{
        rgb::{RgbCommand, RgbDriver, RgbMode, RgbPattern},
        split::MasterToSlave,
    },
    hooks::interface::RgbHooks,
};

use super::channels::{rgb::RGB_CHANNEL, split::M2sTx};
use blinksy::{
    color::{ColorCorrection, IntoColor, LedRgb, LinearSrgb},
    layout::Layout2d,
    pattern::Pattern,
    patterns::rainbow::{Rainbow, RainbowParams},
};

pub async fn start<Layout: Layout2d, Driver: RgbDriver>(
    driver: Option<Driver>,
    mut hook: impl RgbHooks,
    m2s_tx: Option<M2sTx<'_>>,
) {
    let Some(mut driver) = driver else {
        debug!("No rgb");
        return;
    };

    hook.on_rgb_init(&mut driver).await;

    let mut current_rgb_mode = RgbMode::Off;
    let mut brightness = 1.0;
    let color_correction = ColorCorrection::default();
    loop {
        let res = select(RGB_CHANNEL.receive(), async {
            hook.on_rgb_process(&mut driver, &mut current_rgb_mode)
                .await;

            match &current_rgb_mode {
                RgbMode::Off => {
                    let _ = driver
                        .write(core::iter::repeat_n(
                            LedRgb::from_linear_srgb(
                                LinearSrgb::new(0.0, 0.0, 0.0),
                                brightness,
                                color_correction,
                            ),
                            Layout::PIXEL_COUNT,
                        ))
                        .await;
                }
                RgbMode::SolidColor(r, g, b) => {
                    let _ = driver
                        .write(core::iter::repeat_n(
                            LedRgb::from_linear_srgb(
                                LinearSrgb::new(
                                    (*r as f32) / 255.0,
                                    (*g as f32) / 255.0,
                                    (*b as f32) / 255.0,
                                ),
                                brightness,
                                color_correction,
                            ),
                            Layout::PIXEL_COUNT,
                        ))
                        .await;
                }
                RgbMode::Pattern(pat) => {
                    let interval = Duration::from_millis(16);
                    let mut i = 0;
                    let mut t = embassy_time::Ticker::every(interval);

                    macro_rules! process_pattern {
                        ($pattern_ty:ty, $params:expr) => {{
                            let pattern = <$pattern_ty as Pattern<_, Layout>>::new($params);
                            loop {
                                t.next().await;
                                i += 1;
                                let led_data = <$pattern_ty as Pattern<_, Layout>>::tick(
                                    &pattern,
                                    (i * interval).as_millis(),
                                )
                                .map(|color| {
                                    let srgb: LinearSrgb = color.into_color();
                                    LedRgb::from_linear_srgb(srgb, brightness, color_correction)
                                });

                                let _ = driver.write(led_data).await;
                            }
                        }};
                    }

                    match pat {
                        RgbPattern::Rainbow(time_scalar, position_scalar) => {
                            process_pattern!(
                                Rainbow,
                                RainbowParams {
                                    time_scalar: *time_scalar,
                                    position_scalar: *position_scalar,
                                }
                            );
                        }
                    }
                }
                RgbMode::Custom => {
                    // TODO: Implement this
                }
            }
            core::future::pending::<()>().await;
        })
        .await;

        if let Either::First(new_ctrl) = res {
            if let Some(m2s_tx) = m2s_tx {
                m2s_tx.send(MasterToSlave::Rgb(new_ctrl.clone())).await;
            }
            match new_ctrl {
                RgbCommand::Start(rgb_mode) => {
                    current_rgb_mode = rgb_mode;
                }
                RgbCommand::Reset => {}
                RgbCommand::Brightness(mut brightness_value) => {
                    if brightness_value > 1.0 {
                        brightness_value = 1.0;
                    }
                    brightness = brightness_value;
                }
            }
        }
    }
}
