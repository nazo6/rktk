use embassy_futures::select::{Either, select};
use embassy_time::Duration;

use crate::{
    drivers::interface::{rgb::RgbDriver, split::MasterToSlave},
    hooks::interface::RgbHooks,
};

use super::channels::{rgb::RGB_CHANNEL, split::M2sTx};
use blinksy::{
    color::{ColorCorrection, IntoColor, LinearSrgb},
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

    // FIXME: Demo
    let pattern = <Rainbow as Pattern<_, Layout>>::new(RainbowParams::default());

    loop {
        let res = select(RGB_CHANNEL.receive(), async {
            let interval = Duration::from_millis(16);
            let mut i = 0;
            let mut t = embassy_time::Ticker::every(interval);
            loop {
                t.next().await;
                i += 1;
                let led_data =
                    <Rainbow as Pattern<_, Layout>>::tick(&pattern, (i * interval).as_millis())
                        .map(|color| {
                            let srgb: LinearSrgb = color.into_color();
                            srgb
                        });
                let _ = driver
                    .write(led_data, 0.0, ColorCorrection::default())
                    .await;
            }
        })
        .await;

        #[allow(irrefutable_let_patterns)]
        if let Either::First(new_ctrl) = res {
            let _ = m2s_tx.send(MasterToSlave::Rgb(new_ctrl.clone())).await;
        }
    }
}
