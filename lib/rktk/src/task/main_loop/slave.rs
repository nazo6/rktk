use embassy_futures::join::join3;
use embassy_time::Timer;

use crate::{
    config::static_config::{SCAN_INTERVAL_KEYBOARD, SCAN_INTERVAL_MOUSE},
    interface::{
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster},
    },
    task::backlight::BACKLIGHT_CTRL,
};

use super::{M2sRx, S2mTx};

pub async fn start<KS: KeyscanDriver, M: MouseDriver>(
    s2m_tx: S2mTx<'_>,
    m2s_rx: M2sRx<'_>,
    mut key_scanner: KS,
    mut mouse: Option<M>,
) {
    crate::print!("Slave start");
    join3(
        async {
            if let Some(mouse) = &mut mouse {
                loop {
                    let start = embassy_time::Instant::now();

                    if let Ok(data) = mouse.read().await {
                        if data != (0, 0) {
                            let e = SlaveToMaster::Mouse {
                                // x and y are swapped
                                x: data.0,
                                y: data.1,
                            };
                            s2m_tx.send(e).await;
                        }
                    }

                    let took = start.elapsed();
                    if took < SCAN_INTERVAL_MOUSE {
                        Timer::after(SCAN_INTERVAL_MOUSE - took).await;
                    }
                }
            }
        },
        async {
            loop {
                let start = embassy_time::Instant::now();

                let key_events = key_scanner.scan().await;

                for event in key_events {
                    let event = if event.pressed {
                        SlaveToMaster::Pressed(event.row, event.col)
                    } else {
                        SlaveToMaster::Released(event.row, event.col)
                    };

                    s2m_tx.send(event).await;
                }

                let took = start.elapsed();
                if took < SCAN_INTERVAL_KEYBOARD {
                    Timer::after(SCAN_INTERVAL_KEYBOARD - took).await;
                }
            }
        },
        async {
            loop {
                let data = m2s_rx.receive().await;
                match data {
                    MasterToSlave::Backlight(ctrl) => {
                        let _ = BACKLIGHT_CTRL.try_send(ctrl);
                    }
                    MasterToSlave::Message(_) => {}
                }
            }
        },
    )
    .await;
}