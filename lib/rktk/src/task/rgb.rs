use embassy_futures::select::{Either, select};
use embassy_time::Duration;
use rktk_log::debug;

use crate::{
    config::schema::DynamicConfig,
    drivers::interface::{
        rgb::{RgbCommand, RgbDriver, RgbMode, RgbPattern},
        split::MasterToSlave,
    },
    hooks::interface::RgbHooks,
};

use super::channels::{rgb::RGB_CHANNEL, split::M2sTx};
use blinksy::{
    color::{ColorCorrection, IntoColor, LinearSrgb},
    layout::Layout2d,
    pattern::Pattern,
    patterns::{
        noise::{Noise2d, NoiseParams, noise_fns::Perlin},
        rainbow::{Rainbow, RainbowParams},
    },
};

pub async fn start<Layout: Layout2d, Driver: RgbDriver>(
    config: &'static DynamicConfig,
    driver: Option<Driver>,
    mut hook: impl RgbHooks,
    m2s_tx: Option<M2sTx<'_>>,
) {
    let Some(mut driver) = driver else {
        debug!("No rgb");
        return;
    };

    hook.on_rgb_init(&mut driver, m2s_tx.is_some()).await;

    let mut current_rgb_mode = RgbMode::Off;
    let mut brightness = config.rktk.rgb.default_brightness;
    let color_correction = ColorCorrection::default();
    loop {
        let res = select(RGB_CHANNEL.receive(), async {
            hook.on_rgb_process(&mut driver, &mut current_rgb_mode)
                .await;

            match &current_rgb_mode {
                RgbMode::Off => {
                    let _ = driver
                        .write(
                            core::iter::repeat_n(
                                LinearSrgb::new(0.0, 0.0, 0.0),
                                Layout::PIXEL_COUNT,
                            ),
                            brightness,
                            color_correction,
                        )
                        .await;
                }
                RgbMode::SolidColor(r, g, b) => {
                    let _ = driver
                        .write(
                            core::iter::repeat_n(
                                LinearSrgb::new(
                                    (*r as f32) / 255.0,
                                    (*g as f32) / 255.0,
                                    (*b as f32) / 255.0,
                                ),
                                Layout::PIXEL_COUNT,
                            ),
                            brightness,
                            color_correction,
                        )
                        .await;
                }
                RgbMode::Pattern(pat) => {
                    let interval = Duration::from_millis(config.rktk.rgb.pattern_update_interval);
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
                                    srgb
                                });

                                let _ = driver.write(led_data, brightness, color_correction).await;
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
                        RgbPattern::NoisePerlin => {
                            process_pattern!(Noise2d<Perlin>, NoiseParams::default());
                        }
                    }
                }
                RgbMode::Custom => {
                    hook.custom_rgb(&mut driver, brightness).await;
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
                RgbCommand::Brightness(brightness_value) => {
                    brightness = brightness_value.clamp(0.0, 1.0);
                }
                RgbCommand::BrightnessDelta(delta) => {
                    brightness += delta;
                    brightness = brightness.clamp(0.0, 1.0);
                }
            }
        }
    }
}
