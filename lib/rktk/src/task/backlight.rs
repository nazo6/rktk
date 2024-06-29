use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};
use smart_leds::RGB8;

use crate::interface::backlight::{BacklightCtrl, BacklightDriver, BacklightMode};

pub(super) static BACKLIGHT_CTRL: Channel<CriticalSectionRawMutex, BacklightCtrl, 3> =
    Channel::new();

pub async fn start<const BACKLIGHT_COUNT: usize>(mut bl: impl BacklightDriver) {
    loop {
        let ctrl = BACKLIGHT_CTRL.receive().await;
        match ctrl {
            BacklightCtrl::Start(led_animation) => {
                match led_animation {
                    BacklightMode::Rainbow => {
                        //
                    }
                    BacklightMode::Blink => {
                        //
                    }
                    BacklightMode::SolidColor(r, g, b) => {
                        let color = (r, g, b).into();
                        let data = [color; BACKLIGHT_COUNT];
                        bl.write(&data).await;
                    }
                }
            }
            BacklightCtrl::Reset => {
                let data = [RGB8::default(); BACKLIGHT_COUNT];
                bl.write(&data).await;
            }
        }
    }
}
