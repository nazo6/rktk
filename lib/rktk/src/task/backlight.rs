use smart_leds::RGB8;

use crate::{
    drivers::interface::backlight::{BacklightCommand, BacklightDriver, BacklightMode},
    hooks::interface::BacklightHooks,
};

use super::channels::backlight::BACKLIGHT_CHANNEL;

pub async fn start<const BACKLIGHT_COUNT: usize>(
    mut bl: impl BacklightDriver,
    mut hook: impl BacklightHooks,
) {
    hook.on_backlight_init(&mut bl).await;

    loop {
        let ctrl = BACKLIGHT_CHANNEL.receive().await;
        let mut rgb_data = match &ctrl {
            BacklightCommand::Start(led_animation) => match led_animation {
                BacklightMode::Rainbow => None,
                BacklightMode::Blink => None,
                BacklightMode::SolidColor(r, g, b) => {
                    let color = (*r, *g, *b).into();
                    Some([color; BACKLIGHT_COUNT])
                }
            },
            BacklightCommand::Reset => Some([RGB8::default(); BACKLIGHT_COUNT]),
        };

        hook.on_backlight_process(&mut bl, &ctrl, &mut rgb_data)
            .await;

        if let Some(rgb_data) = rgb_data {
            let _ = bl.write(&rgb_data).await;
        }
    }
}
