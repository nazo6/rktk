use embassy_futures::select::{Either, select};
use embassy_time::Duration;

use crate::{
    drivers::interface::{
        rgb::{Rgb8, RgbCommand, RgbDriver, RgbMode, RgbPattern},
        split::MasterToSlave,
    },
    hooks::interface::RgbHooks,
};

use super::channels::{rgb::RGB_CHANNEL, split::M2sTx};
use blinksy::{
    color::{ColorCorrection, IntoColor, LinearSrgb},
    dimension::Dim2d,
    layout::Layout2d,
    pattern::Pattern,
    patterns::rainbow::{Rainbow, RainbowParams},
};

pub async fn start<Layout: Layout2d, Driver: RgbDriver>(
    mut driver: Driver,
    mut hook: impl RgbHooks,
    m2s_tx: M2sTx<'_>,
) {
    hook.on_rgb_init(&mut driver).await;

    let mut current_rgb_mode = RgbMode::Off;
    loop {
        let res = select(RGB_CHANNEL.receive(), async {
            hook.on_rgb_process(&mut driver, &mut current_rgb_mode)
                .await;

            match &current_rgb_mode {
                RgbMode::Off => {
                    let _ = driver
                        .write(
                            core::iter::repeat_n(Rgb8::new(0, 0, 0), Layout::PIXEL_COUNT),
                            0.0,
                            ColorCorrection::default(),
                        )
                        .await;
                }
                RgbMode::SolidColor(r, g, b) => {
                    let _ = driver
                        .write(
                            core::iter::repeat_n(Rgb8::new(*r, *g, *b), Layout::PIXEL_COUNT),
                            0.0,
                            ColorCorrection::default(),
                        )
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
                                    srgb.into()
                                });
                                let _ = driver
                                    .write(led_data, 0.0, ColorCorrection::default())
                                    .await;
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
            let _ = m2s_tx.send(MasterToSlave::Rgb(new_ctrl.clone())).await;
            match new_ctrl {
                RgbCommand::Start(rgb_mode) => {
                    current_rgb_mode = rgb_mode;
                }
                RgbCommand::Reset => {}
            }
        }
    }
}

struct TimeGradPat;
impl<L: Layout2d> Pattern<Dim2d, L> for TimeGradPat {
    type Params = ();
    type Color = LinearSrgb;

    fn new(_params: Self::Params) -> Self {
        Self
    }

    fn tick(&self, time_in_ms: u64) -> impl Iterator<Item = Self::Color> {
        (0..L::PIXEL_COUNT).map(move |i| {
            LinearSrgb::new(
                ((time_in_ms / 5 + i as u64 * 5) % 255) as f32 / 255.0,
                (255 - (time_in_ms / 5 + i as u64 * 5) % 255) as f32 / 255.0,
                0.0,
            )
        })
    }
}
