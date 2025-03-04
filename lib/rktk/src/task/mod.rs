//! Program entrypoint.

use crate::{
    config::constant::RKTK_CONFIG,
    drivers::interface::{
        ble::BleDriver,
        debounce::DebounceDriver,
        display::DisplayDriver,
        dongle::{DongleData, DongleDriver},
        encoder::EncoderDriver,
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        rgb::RgbDriver,
        split::SplitDriver,
        storage::StorageDriver,
        usb::UsbDriver,
        BackgroundTask as _, DriverBuilder, DriverBuilderWithTask,
    },
    hooks::Hooks,
};
use crate::{
    config::keymap::Keymap,
    drivers::{interface::system::SystemDriver, Drivers},
    hooks::interface::*,
    utils::sjoin,
};
use embassy_time::Duration;
use rktk_log::{debug, helper::Debug2Format, info};

pub(crate) mod channels;
pub mod display;
#[cfg(feature = "log")]
mod logger;
pub(crate) mod main_loop;

/// Receives configs and executes the main process of the keyboard.
///
/// # Parameters
/// - `drivers`: Drivers for the keyboard.
/// - `key_config`: Key configuration such as keymaps.
/// - `hooks`: Hooks for the keyboard. See [`Hooks`] for detail.
#[allow(clippy::type_complexity)]
pub async fn start<
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Encoder: EncoderDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    Split: SplitDriver,
    Rgb: RgbDriver,
    System: SystemDriver,
    Storage: StorageDriver,
    Mouse: MouseDriver,
    Display: DisplayDriver,
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display> + 'static,
    UsbBuilder: DriverBuilderWithTask<Driver = Usb>,
    BleBuilder: DriverBuilderWithTask<Driver = Ble>,
    CH: CommonHooks,
    MH: MasterHooks,
    SH: SlaveHooks,
    BH: RgbHooks,
>(
    drivers: Drivers<
        KeyScan,
        Debounce,
        Encoder,
        Ble,
        Usb,
        Split,
        Rgb,
        System,
        Storage,
        Mouse,
        Display,
        MouseBuilder,
        DisplayBuilder,
        UsbBuilder,
        BleBuilder,
    >,
    key_config: Keymap,
    hooks: Hooks<CH, MH, SH, BH>,
) {
    #[cfg(feature = "log")]
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
        drivers.mouse_builder.is_some(),
        drivers.display_builder.is_some(),
    );

    drivers
        .system
        .double_reset_usb_boot(Duration::from_millis(RKTK_CONFIG.double_tap_threshold))
        .await;

    sjoin::join!(
        async move {
            let mouse = if let Some(mouse_builder) = drivers.mouse_builder {
                debug!("Mouse init");
                match mouse_builder.build().await {
                    Ok(mut mouse) => {
                        let _ = mouse.set_cpi(RKTK_CONFIG.default_cpi).await;
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
                        drivers.debounce,
                        drivers.encoder,
                        mouse,
                        drivers.storage,
                        drivers.split,
                        drivers.rgb,
                        key_config,
                        hooks,
                    )
                    .await;
                },
                async {
                    if let Some(usb_task) = usb_task {
                        usb_task.run().await
                    }
                },
                async {
                    if let Some(ble_task) = ble_task {
                        ble_task.run().await
                    }
                }
            );
        },
        async move {
            if let Some(display_builder) = drivers.display_builder {
                display::start(display_builder).await;
            }
        }
    );
}

pub async fn dongle_start<
    Usb: UsbDriver,
    Dongle: DongleDriver,
    Display: DisplayDriver,
    DisplayBuilder: DriverBuilder<Output = Display> + 'static,
>(
    usb: impl DriverBuilderWithTask<Driver = Usb>,
    dongle: impl DriverBuilderWithTask<Driver = Dongle>,
    mut hooks: impl dongle::DongleHooks,
    display: Option<DisplayBuilder>,
) {
    let (usb, usb_task) = usb.build().await.unwrap();
    let (mut dongle, dongle_task) = dongle.build().await.unwrap();

    let main_task = async move {
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
    };

    sjoin::join!(main_task, usb_task.run(), dongle_task.run(), async move {
        if let Some(display_builder) = display {
            display::start(display_builder).await;
        }
    });
}
