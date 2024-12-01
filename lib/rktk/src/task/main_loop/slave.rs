use embassy_futures::join::join3;
use embassy_time::Timer;

use crate::{
    config::static_config::SCAN_INTERVAL_MOUSE,
    drivers::interface::{
        debounce::DebounceDriver,
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster},
    },
    hooks::interface::SlaveHooks,
    task::channels::{
        backlight::BACKLIGHT_CHANNEL,
        split::{M2sRx, S2mTx},
    },
};

pub async fn start<KS: KeyscanDriver, M: MouseDriver, DB: DebounceDriver, SH: SlaveHooks>(
    s2m_tx: S2mTx<'_>,
    m2s_rx: M2sRx<'_>,
    mut keyscan: KS,
    mut debounce: Option<DB>,
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
                let mut buf = heapless::Vec::<_, 32>::new();
                keyscan
                    .scan(|event| {
                        let _ = buf.push(event);
                    })
                    .await;
                for event in buf {
                    if let Some(debounce) = &mut debounce {
                        if debounce.should_ignore_event(&event, embassy_time::Instant::now()) {
                            return;
                        }
                    }

                    let event = if event.pressed {
                        SlaveToMaster::Pressed(event.row, event.col)
                    } else {
                        SlaveToMaster::Released(event.row, event.col)
                    };

                    s2m_tx.send(event).await;
                }
            }
        },
        async {
            loop {
                let data = m2s_rx.receive().await;
                match data {
                    MasterToSlave::Backlight(ctrl) => {
                        let _ = BACKLIGHT_CHANNEL.try_send(ctrl);
                    }
                    MasterToSlave::Message(_) => {}
                }
            }
        },
    )
    .await;
}
