use embassy_futures::join::join3;
use embassy_time::Timer;
use rktk_log::debug;

use crate::{
    config::constant::{SCAN_INTERVAL_KEYBOARD, SCAN_INTERVAL_MOUSE},
    drivers::interface::{
        debounce::DebounceDriver,
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster},
    },
    hooks::interface::SlaveHooks,
    task::channels::{
        rgb::RGB_CHANNEL,
        split::{M2sRx, S2mTx},
    },
};

pub async fn start<KS: KeyscanDriver, M: MouseDriver, DB: DebounceDriver, SH: SlaveHooks>(
    s2m_tx: S2mTx<'_>,
    m2s_rx: M2sRx<'_>,
    mut keyscan: KS,
    debounce: &mut Option<DB>,
    mut mouse: Option<M>,
    mut slave_hooks: SH,
) {
    crate::print!("Slave start");

    slave_hooks
        .on_slave_init(&mut keyscan, mouse.as_mut())
        .await;

    join3(
        async {
            if let Some(mouse) = &mut mouse {
                debug!("mouse start");
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
            debug!("keyscan start");
            loop {
                Timer::after(SCAN_INTERVAL_KEYBOARD).await;

                keyscan
                    .scan(|event| {
                        debug!("keyscan event: {:?}", event);
                        if let Some(debounce) = debounce.as_mut() {
                            if debounce.should_ignore_event(&event, embassy_time::Instant::now()) {
                                debug!("keyscan event ignored");
                                return;
                            }
                        }

                        let event = if event.pressed {
                            SlaveToMaster::Pressed(event.row, event.col)
                        } else {
                            SlaveToMaster::Released(event.row, event.col)
                        };

                        let _ = s2m_tx.try_send(event);
                    })
                    .await;
            }
        },
        async {
            debug!("split start");
            loop {
                let data = m2s_rx.receive().await;
                debug!("split data recv: {:?}", data);
                match data {
                    MasterToSlave::Rgb(ctrl) => {
                        let _ = RGB_CHANNEL.try_send(ctrl);
                    }
                    MasterToSlave::Message(_) => {}
                }
            }
        },
    )
    .await;
}
