use smart_leds::RGB8;

use crate::{
    drivers::interface::rgb::{RgbCommand, RgbDriver, RgbMode},
    hooks::interface::RgbHooks,
};

use super::channels::rgb::RGB_CHANNEL;

pub async fn start<const LED_COUNT: usize>(mut bl: impl RgbDriver, mut hook: impl RgbHooks) {
    hook.on_rgb_init(&mut bl).await;

    loop {
        let ctrl = RGB_CHANNEL.receive().await;
        let mut rgb_data = match &ctrl {
            RgbCommand::Start(led_animation) => match led_animation {
                RgbMode::Rainbow => None,
                RgbMode::Blink => None,
                RgbMode::SolidColor(r, g, b) => {
                    let color = (*r, *g, *b).into();
                    Some([color; LED_COUNT])
                }
            },
            RgbCommand::Reset => Some([RGB8::default(); LED_COUNT]),
        };

        hook.on_rgb_process(&mut bl, &ctrl, &mut rgb_data).await;

        if let Some(rgb_data) = rgb_data {
            let _ = bl.write(&rgb_data).await;
        }
    }
}
