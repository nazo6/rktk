//! Program entrypoint.

use crate::{
    config::Hand,
    drivers::interface::{
        debounce::DebounceDriver, display::DisplayDriver, encoder::EncoderDriver,
        keyscan::KeyscanDriver, mouse::MouseDriver, rgb::RgbDriver, split::SplitDriver,
        storage::StorageDriver, usb::UsbReporterDriverBuilder,
        wireless::WirelessReporterDriverBuilder,
    },
    hooks::AllHooks,
};
use crate::{
    drivers::{Drivers, interface::system::SystemDriver},
    hooks::interface::*,
    utils::sjoin,
};
use display::DisplayConfig;
use embassy_futures::join::{join, join5};
use embassy_time::Duration;
use rktk_log::{debug, info};

pub(crate) mod channels;
// `display` module is public as internally used by macros
pub mod display;
mod initializers;
#[cfg(feature = "rrp-log")]
mod logger;
mod master;
mod rgb;
mod slave;
mod split_handler;

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
    H: AllHooks,
    DC: DisplayConfig + 'static,
    RL: blinksy::layout::Layout2d + 'static,
>(
    #[allow(
        unused_variables,
        reason = "`spawner` is unused when `alloc` is disabled"
    )]
    spawner: embassy_executor::Spawner,
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
    hooks: H,
    mut opts: crate::config::RktkOpts<DC, RL>,
) {
    #[cfg(feature = "rrp-log")]
    {
        debug!("log init");
        critical_section::with(|_| unsafe {
            let _ = log::set_logger_racy(&logger::RRP_LOGGER);
            log::set_max_level_racy(log::LevelFilter::Info);
        });
    }

    info!("Booting rktk",);

    drivers
        .system
        .double_reset_usb_boot(Duration::from_millis(opts.config.rktk.split_usb_timeout))
        .await;

    let mut hooks = hooks.destructure();

    sjoin::join!(
        spawner,
        async {
            initializers::init_mouse(&mut drivers.mouse, opts.config).await;
            let ((wireless, wireless_task), (usb, usb_task)) =
                initializers::init_reporters(drivers.ble_builder, drivers.usb_builder).await;

            sjoin::join!(
                spawner,
                async {
                    let hand = opts.hand.unwrap_or(Hand::Left);
                    crate::utils::display_state!(Hand, Some(hand));

                    let role =
                        initializers::init_split(opts.config, drivers.split, &usb, &wireless).await;

                    hooks
                        .common
                        .on_init(
                            hand,
                            &mut drivers.keyscan,
                            drivers.mouse.as_mut(),
                            drivers.storage.as_mut(),
                        )
                        .await;

                    crate::utils::display_state!(Master, Some(role.is_master()));

                    match role {
                        initializers::KeyboardRoleRes::Master {
                            sender,
                            receiver,
                            task,
                        } => {
                            info!("Master start");
                            sjoin::join!(
                                spawner,
                                async {
                                    let config_store =
                                        master::utils::init_storage(drivers.storage).await;
                                    let state = master::utils::load_state(
                                        &opts.config.key_manager,
                                        &config_store,
                                        opts.keymap,
                                    )
                                    .await;

                                    hooks
                                        .master
                                        .on_master_init(
                                            &mut drivers.keyscan,
                                            drivers.mouse.as_mut(),
                                        )
                                        .await;

                                    join(
                                        join5(
                                            master::report::report_task(
                                                opts.config,
                                                &drivers.system,
                                                &state,
                                                &config_store,
                                                &wireless,
                                                &usb,
                                                hooks.master,
                                            ),
                                            master::handle_slave::start(
                                                opts.config,
                                                hand,
                                                receiver,
                                            ),
                                            master::handle_keyboard::start(
                                                opts.config,
                                                hand,
                                                drivers.keyscan,
                                                &mut drivers.debounce,
                                            ),
                                            master::handle_mouse::start(opts.config, drivers.mouse),
                                            master::handle_encoder::start(&mut drivers.encoder),
                                        ),
                                        async {
                                            #[cfg(feature = "rrp")]
                                            master::rrp_server::start(
                                                opts.config,
                                                &usb,
                                                &wireless,
                                                &state,
                                                &config_store,
                                            )
                                            .await;
                                        },
                                    )
                                    .await;
                                },
                                rgb::start::<RL, _>(
                                    opts.config,
                                    drivers.rgb,
                                    hooks.rgb,
                                    Some(sender)
                                ),
                                async move {
                                    if let Some(task) = task {
                                        task.await;
                                    }
                                }
                            );
                        }
                        initializers::KeyboardRoleRes::Slave {
                            sender,
                            receiver,
                            task,
                        } => {
                            debug!("Slave start");
                            sjoin::join!(
                                spawner,
                                async move {
                                    slave::start(
                                        opts.config,
                                        sender,
                                        receiver,
                                        drivers.keyscan,
                                        &mut drivers.debounce,
                                        drivers.mouse,
                                        hooks.slave,
                                    )
                                    .await
                                },
                                rgb::start::<RL, _>(opts.config, drivers.rgb, hooks.rgb, None),
                                async move {
                                    if let Some(task) = task {
                                        task.await;
                                    }
                                }
                            );
                        }
                    }
                },
                async {
                    if let Some(usb_task) = usb_task {
                        usb_task.await
                    }
                },
                async {
                    if let Some(wireless_task) = wireless_task {
                        wireless_task.await
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
