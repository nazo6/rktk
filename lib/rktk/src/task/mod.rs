//! Program entrypoint.

use crate::{
    config::{CONST_CONFIG, Hand},
    drivers::interface::{
        debounce::DebounceDriver, display::DisplayDriver, encoder::EncoderDriver,
        keyscan::KeyscanDriver, mouse::MouseDriver, reporter::ReporterDriver, rgb::RgbDriver,
        split::SplitDriver, storage::StorageDriver, usb::UsbReporterDriverBuilder,
        wireless::WirelessReporterDriverBuilder,
    },
    hooks::Hooks,
};
use crate::{
    drivers::{Drivers, interface::system::SystemDriver},
    hooks::interface::*,
    utils::sjoin,
};
use channels::{
    report::ENCODER_EVENT_REPORT_CHANNEL,
    split::{M2S_CHANNEL, S2M_CHANNEL},
};
use display::DisplayConfig;
use embassy_futures::{
    join::{join, join4},
    select::{Either, select},
};
use embassy_time::{Duration, Timer};
use rktk_log::{debug, helper::Debug2Format, info, warn};

pub(crate) mod channels;
pub(crate) mod display;
#[cfg(feature = "rrp-log")]
mod logger;
pub(crate) mod master;
mod rgb;
pub(crate) mod slave;
pub(crate) mod split_handler;

/// Runs rktk with the given drivers and key configuration.
///
/// # Parameters
/// - `drivers`: Drivers for the keyboard.
/// - `hooks`: Hooks for the keyboard. See [`Hooks`] for detail.
/// - `opts`: Other options such as keymap. See [`crate::config`] for detail.
#[allow(clippy::type_complexity)]
pub async fn start<
    System: SystemDriver,
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Encoder: EncoderDriver,
    Rgb: RgbDriver,
    Storage: StorageDriver,
    Split: SplitDriver,
    Ble: WirelessReporterDriverBuilder,
    Usb: UsbReporterDriverBuilder,
    Display: DisplayDriver,
    Mouse: MouseDriver,
    //
    CH: CommonHooks,
    MH: MasterHooks,
    SH: SlaveHooks,
    BH: RgbHooks,
    //
    DC: DisplayConfig + 'static,
>(
    mut drivers: Drivers<
        System,
        KeyScan,
        Debounce,
        Encoder,
        Rgb,
        Storage,
        Split,
        Ble,
        Usb,
        Display,
        Mouse,
    >,
    mut hooks: Hooks<CH, MH, SH, BH>,
    mut opts: crate::config::RktkOpts<DC>,
) {
    #[cfg(feature = "rrp-log")]
    {
        debug!("log init");
        critical_section::with(|_| unsafe {
            let _ = log::set_logger_racy(&logger::RRP_LOGGER);
            log::set_max_level_racy(log::LevelFilter::Info);
        });
    }

    info!(
        "RKTK Starting... (rgb: {}, ble: {}, usb: {}, storage: {}, mouse: {}, display: {})",
        drivers.rgb.is_some(),
        drivers.ble_builder.is_some(),
        drivers.usb_builder.is_some(),
        drivers.storage.is_some(),
        drivers.mouse.is_some(),
        drivers.display.is_some(),
    );

    drivers
        .system
        .double_reset_usb_boot(Duration::from_millis(opts.config.rktk.split_usb_timeout))
        .await;

    sjoin::join!(
        async {
            let mut mouse = if let Some(mut mouse) = drivers.mouse {
                debug!("Mouse init");

                match mouse.init().await {
                    Ok(_) => {
                        let _ = mouse.set_cpi(opts.config.rktk.default_cpi).await;
                        Some(mouse)
                    }
                    Err(e) => {
                        rktk_log::warn!("Failed to build mouse driver: {:?}", Debug2Format(&e));
                        None
                    }
                }
            } else {
                debug!("no mouse");
                None
            };

            crate::utils::display_state!(MouseAvailable, mouse.is_some());

            let (ble, ble_task) = if let Some(ble_builder) = drivers.ble_builder {
                debug!("BLE init");
                let (ble, ble_task) = ble_builder.build().await.expect("Failed to build ble");
                (Some(ble), Some(ble_task))
            } else {
                debug!("no BLE driver");
                (None, None)
            };

            let (usb, usb_task) = if let Some(usb_builder) = drivers.usb_builder {
                debug!("USB init");
                let (usb, usb_task) = usb_builder.build().await.expect("Failed to build usb");
                (Some(usb), Some(usb_task))
            } else {
                debug!("no USB driver");
                (None, None)
            };

            sjoin::join!(
                async {
                    let hand = opts.hand.unwrap_or(Hand::Left);

                    crate::utils::display_state!(Hand, Some(hand));

                    let split = if let Some(mut split) = drivers.split {
                        match split.init().await {
                            Ok(_) => Some(split),
                            Err(e) => {
                                rktk_log::error!(
                                    "Failed to initialize split: {:?}",
                                    Debug2Format(&e)
                                );
                                None
                            }
                        }
                    } else {
                        None
                    };

                    let usb_available = if let Some(usb) = &usb {
                        match select(
                            usb.wait_ready(),
                            Timer::after(Duration::from_millis(opts.config.rktk.split_usb_timeout)),
                        )
                        .await
                        {
                            Either::First(_) => true,
                            Either::Second(_) => false,
                        }
                    } else {
                        false
                    };

                    let is_master = split.is_none() || usb_available || ble.is_some();

                    hooks
                        .common
                        .on_init(
                            hand,
                            &mut drivers.keyscan,
                            mouse.as_mut(),
                            drivers.storage.as_mut(),
                        )
                        .await;

                    crate::utils::display_state!(Master, Some(is_master));

                    let s2m_tx = S2M_CHANNEL.sender();
                    let s2m_rx = S2M_CHANNEL.receiver();

                    let m2s_tx = M2S_CHANNEL.sender();
                    let m2s_rx = M2S_CHANNEL.receiver();

                    let rgb_m2s_tx = if is_master {
                        Some(M2S_CHANNEL.sender())
                    } else {
                        None
                    };

                    sjoin::join!(
                        async move {
                            if is_master {
                                debug!("master start");
                                let config_store =
                                    master::utils::init_storage(drivers.storage).await;
                                let state = master::utils::load_state(
                                    &opts.config.key_manager,
                                    &config_store,
                                    opts.keymap,
                                )
                                .await;

                                info!("Master side task start");

                                hooks
                                    .master
                                    .on_master_init(&mut drivers.keyscan, mouse.as_mut())
                                    .await;

                                join(
                                    join(
                                        master::report::report_task(
                                            opts.config,
                                            &drivers.system,
                                            &state,
                                            &config_store,
                                            &ble,
                                            &usb,
                                            hooks.master,
                                        ),
                                        join4(
                                            master::handle_slave::start(opts.config, hand, s2m_rx),
                                            master::handle_keyboard::start(
                                                opts.config,
                                                hand,
                                                drivers.keyscan,
                                                &mut drivers.debounce,
                                            ),
                                            master::handle_mouse::start(opts.config, mouse),
                                            async {
                                                if let Some(encoder) = &mut drivers.encoder {
                                                    loop {
                                                        let (id, dir) = encoder.read_wait().await;
                                                        if ENCODER_EVENT_REPORT_CHANNEL
                                                            .try_send((id, dir))
                                                            .is_err()
                                                        {
                                                            warn!("enc full");
                                                        }
                                                    }
                                                }
                                            },
                                        ),
                                    ),
                                    async {
                                        #[cfg(feature = "rrp")]
                                        master::rrp_server::start(
                                            opts.config,
                                            &usb,
                                            &ble,
                                            &state,
                                            &config_store,
                                        )
                                        .await;
                                    },
                                )
                                .await;
                            } else {
                                debug!("slave start");
                                slave::start(
                                    opts.config,
                                    s2m_tx,
                                    m2s_rx,
                                    drivers.keyscan,
                                    &mut drivers.debounce,
                                    mouse,
                                    hooks.slave,
                                )
                                .await;
                            }
                        },
                        async move {
                            if let Some(split) = split {
                                debug!("split init");
                                if is_master {
                                    split_handler::start(split, s2m_tx, m2s_rx, is_master).await;
                                } else {
                                    split_handler::start(split, m2s_tx, s2m_rx, is_master).await;
                                }
                            } else {
                                debug!("no split");
                            }
                        },
                        async move {
                            if let Some(rgb) = drivers.rgb {
                                debug!("rgb init");
                                match hand {
                                    Hand::Right => {
                                        rgb::start::<{ CONST_CONFIG.keyboard.right_rgb_count }>(
                                            rgb, hooks.rgb, rgb_m2s_tx,
                                        )
                                        .await
                                    }
                                    Hand::Left => {
                                        rgb::start::<{ CONST_CONFIG.keyboard.left_rgb_count }>(
                                            rgb, hooks.rgb, rgb_m2s_tx,
                                        )
                                        .await
                                    }
                                }
                            } else {
                                debug!("no rgb");
                            }
                        }
                    );
                },
                async {
                    if let Some(usb_task) = usb_task {
                        usb_task.await
                    }
                },
                async {
                    if let Some(ble_task) = ble_task {
                        ble_task.await
                    }
                }
            );
        },
        async move {
            if let Some(mut display) = drivers.display {
                display::start(&mut display, &mut opts.display).await;
            }
        }
    );
}
