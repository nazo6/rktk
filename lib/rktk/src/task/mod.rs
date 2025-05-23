//! Program entrypoint.

use crate::{
    drivers::interface::{
        debounce::DebounceDriver,
        display::DisplayDriver,
        dongle::{DongleData, DongleDriver, DongleDriverBuilder},
        encoder::EncoderDriver,
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        reporter::ReporterDriver,
        rgb::RgbDriver,
        split::SplitDriver,
        storage::StorageDriver,
        usb::UsbReporterDriverBuilder,
        wireless::WirelessReporterDriverBuilder,
    },
    hooks::Hooks,
    interface::Hand,
};
use crate::{
    drivers::{Drivers, interface::system::SystemDriver},
    hooks::interface::*,
    utils::sjoin,
};
use display::DisplayConfig;
use embassy_time::Duration;
use rktk_log::{debug, helper::Debug2Format, info};

pub(crate) mod channels;
pub mod display;
#[cfg(feature = "rrp-log")]
mod logger;
pub(crate) mod main_loop;

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
    hooks: Hooks<CH, MH, SH, BH>,
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
            let mouse = if let Some(mut mouse) = drivers.mouse {
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
                    main_loop::start(
                        &drivers.system,
                        ble,
                        usb,
                        drivers.keyscan,
                        &mut drivers.debounce,
                        drivers.encoder,
                        mouse,
                        drivers.storage,
                        drivers.split,
                        drivers.rgb,
                        opts.config,
                        opts.keymap,
                        hooks,
                        opts.hand.unwrap_or(Hand::Left),
                    )
                    .await;
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

/// Runs dongle with the given drivers.
pub async fn dongle_start<D: display::DisplayConfig + 'static>(
    usb: impl UsbReporterDriverBuilder,
    dongle: impl DongleDriverBuilder,
    mut hooks: impl dongle::DongleHooks,
    display: Option<impl DisplayDriver>,
    mut display_config: impl display::DisplayConfig + 'static,
) {
    let (usb, usb_task) = usb.build().await.unwrap();
    let (mut dongle, dongle_task) = dongle.build().await.unwrap();

    sjoin::join!(
        async move {
            loop {
                let data = dongle.recv().await;
                match data {
                    Ok(mut data) => {
                        if !hooks.on_dongle_data(&mut data).await {
                            continue;
                        }
                        match data {
                            DongleData::Keyboard(report) => {
                                if let Err(e) = usb.try_send_keyboard_report(report.into()) {
                                    rktk_log::warn!(
                                        "Failed to send keyboard report: {:?}",
                                        Debug2Format(&e)
                                    );
                                }
                            }
                            DongleData::Mouse(report) => {
                                if let Err(e) = usb.try_send_mouse_report(report.into()) {
                                    rktk_log::warn!(
                                        "Failed to send mouse report: {:?}",
                                        Debug2Format(&e)
                                    );
                                }
                            }
                            DongleData::MediaKeyboard(report) => {
                                if let Err(e) = usb.try_send_media_keyboard_report(report.into()) {
                                    rktk_log::warn!(
                                        "Failed to send media keyboard report: {:?}",
                                        Debug2Format(&e)
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        rktk_log::warn!("Dongle recv fail: {:?}", Debug2Format(&e));
                        embassy_time::Timer::after_millis(100).await;
                        continue;
                    }
                }
            }
        },
        usb_task,
        dongle_task,
        async move {
            if let Some(mut display) = display {
                display::start(&mut display, &mut display_config).await;
            }
        }
    );
}
