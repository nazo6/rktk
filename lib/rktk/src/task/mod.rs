//! Program entrypoint.

use drivers::Drivers;
use embassy_futures::join::{join, join3};
use embassy_time::Duration;

use crate::{
    config::static_config::RKTK_CONFIG,
    hooks::Hooks,
    interface::{
        backlight::BacklightDriver, ble::BleDriver, debounce::DebounceDriver,
        display::DisplayDriver, double_tap::DoubleTapResetDriver, encoder::EncoderDriver,
        keyscan::KeyscanDriver, mouse::MouseDriver, split::SplitDriver, storage::StorageDriver,
        usb::UsbDriver, BackgroundTask as _, DriverBuilder, DriverBuilderWithTask,
    },
    KeyConfig,
};

mod backlight;
pub mod display;
pub mod drivers;
mod logger;
pub(crate) mod main_loop;

/// Receives the [`Drivers`] and executes the main process of the keyboard.
/// This function must not be called more than once.
#[allow(clippy::type_complexity)]
pub async fn start<
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Encoder: EncoderDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    Split: SplitDriver,
    Backlight: BacklightDriver,
    DoubleTapReset: DoubleTapResetDriver,
    Storage: StorageDriver,
    Mouse: MouseDriver,
    Display: DisplayDriver,
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilderWithTask<Driver = Usb>,
    BleBuilder: DriverBuilderWithTask<Driver = Ble>,
    MainHooks: crate::hooks::MainHooks,
    BacklightHooks: crate::hooks::BacklightHooks,
>(
    mut drivers: Drivers<
        KeyScan,
        Split,
        Debounce,
        Ble,
        Usb,
        Backlight,
        Storage,
        Mouse,
        Display,
        MouseBuilder,
        DisplayBuilder,
        UsbBuilder,
        BleBuilder,
        DoubleTapReset,
        Encoder,
    >,
    key_config: KeyConfig,
    hooks: Hooks<MainHooks, BacklightHooks>,
) {
    critical_section::with(|_| unsafe {
        let _ = log::set_logger_racy(&logger::RRP_LOGGER);
        log::set_max_level_racy(log::LevelFilter::Info);
    });

    log::info!(
        "RKTK Starting... (backlight: {}, ble: {}, usb: {}, storage: {}, mouse: {}, display: {})",
        drivers.backlight.is_some(),
        drivers.ble_builder.is_some(),
        drivers.usb_builder.is_some(),
        drivers.storage.is_some(),
        drivers.mouse_builder.is_some(),
        drivers.display_builder.is_some(),
    );

    if let Some(dtr) = &mut drivers.double_tap_reset {
        dtr.execute(Duration::from_millis(RKTK_CONFIG.double_tap_threshold))
            .await;
    }

    join(
        async move {
            if let Some(display_builder) = drivers.display_builder {
                display::start(display_builder).await;
            }
        },
        async {
            let mouse = if let Some(mouse_builder) = drivers.mouse_builder {
                match mouse_builder.build().await {
                    Ok(mut mouse) => {
                        let _ = mouse.set_cpi(RKTK_CONFIG.default_cpi).await;
                        Some(mouse)
                    }
                    Err(e) => {
                        log::warn!("Failed to build mouse driver: {:?}", e);
                        None
                    }
                }
            } else {
                None
            };

            crate::utils::display_state!(MouseAvailable, mouse.is_some());

            let (ble, ble_task) = if let Some(ble_builder) = drivers.ble_builder {
                let (ble, ble_task) = ble_builder.build().await.expect("Failed to build ble");
                (Some(ble), Some(ble_task))
            } else {
                (None, None)
            };

            let (usb, usb_task) = if let Some(usb_builder) = drivers.usb_builder {
                let (usb, usb_task) = usb_builder.build().await.expect("Failed to build usb");
                (Some(usb), Some(usb_task))
            } else {
                (None, None)
            };

            join3(
                async {
                    if let Some(usb_task) = usb_task {
                        usb_task.run().await
                    }
                },
                async {
                    if let Some(ble_task) = ble_task {
                        ble_task.run().await
                    }
                },
                async {
                    main_loop::start(
                        ble,
                        usb,
                        drivers.keyscan,
                        drivers.debounce,
                        drivers.encoder,
                        mouse,
                        drivers.storage,
                        drivers.split,
                        drivers.backlight,
                        key_config,
                        hooks,
                    )
                    .await;
                },
            )
            .await;
        },
    )
    .await;
}
