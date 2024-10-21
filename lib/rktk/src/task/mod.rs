//! Program entrypoint.

use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_time::{Duration, Timer};

use crate::{
    config::static_config::RKTK_CONFIG,
    hooks::Hooks,
    interface::{
        backlight::BacklightDriver, ble::BleDriver, debounce::DebounceDriver,
        display::DisplayDriver, double_tap::DoubleTapResetDriver, keyscan::KeyscanDriver,
        mouse::MouseDriver, split::SplitDriver, storage::StorageDriver, usb::UsbDriver,
        DriverBuilder,
    },
    KeyConfig,
};

mod backlight;
pub mod display;
mod logger;
pub(crate) mod main_loop;

/// All drivers required to run the keyboard.
///
/// Only the `key_scanner` and `usb` drivers are required.
/// For other drivers, if the value is None, it will be handled appropriately.
pub struct Drivers<
    // required drivers
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Split: SplitDriver,
    // builder drivers
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilder<Output = Usb>,
    // optional drivers
    Backlight: BacklightDriver,
    Ble: BleDriver,
    DoubleTapReset: DoubleTapResetDriver,
    Storage: StorageDriver,
    // optional drivers of builder
    Mouse: MouseDriver,
    Display: DisplayDriver,
    Usb: UsbDriver,
> {
    pub double_tap_reset: Option<DoubleTapReset>,
    pub keyscan: KeyScan,
    pub debounce: Debounce,
    pub split: Split,
    pub backlight: Option<Backlight>,
    pub ble: Option<Ble>,
    pub storage: Option<Storage>,

    pub usb_builder: Option<UsbBuilder>,
    pub mouse_builder: Option<MouseBuilder>,
    pub display_builder: Option<DisplayBuilder>,
}

/// Receives the [`Drivers`] and executes the main process of the keyboard.
/// This function must not be called more than once.
pub async fn start<
    KeyScan: KeyscanDriver,
    Debounce: DebounceDriver,
    Split: SplitDriver,
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilder<Output = Usb>,
    Backlight: BacklightDriver,
    Ble: BleDriver,
    DoubleTapReset: DoubleTapResetDriver,
    Storage: StorageDriver,
    Mouse: MouseDriver,
    Display: DisplayDriver,
    Usb: UsbDriver,
    MainHooks: crate::hooks::MainHooks,
    BacklightHooks: crate::hooks::BacklightHooks,
>(
    mut drivers: Drivers<
        KeyScan,
        Debounce,
        Split,
        MouseBuilder,
        DisplayBuilder,
        UsbBuilder,
        Backlight,
        Ble,
        DoubleTapReset,
        Storage,
        Mouse,
        Display,
        Usb,
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
        drivers.ble.is_some(),
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

            match (drivers.ble, drivers.usb_builder) {
                (Some(ble), _) => {
                    main_loop::start(
                        Some(&ble),
                        drivers.keyscan,
                        drivers.debounce,
                        mouse,
                        drivers.storage,
                        drivers.split,
                        drivers.backlight,
                        key_config,
                        hooks,
                    )
                    .await;
                }
                (_, Some(usb_builder)) => {
                    let usb = match select(
                        usb_builder.build(),
                        Timer::after(Duration::from_millis(RKTK_CONFIG.split_usb_timeout)),
                    )
                    .await
                    {
                        Either::First(Ok(usb)) => Some(usb),
                        Either::First(_) => {
                            panic!("Failed to build USB");
                        }
                        Either::Second(_) => None,
                    };
                    main_loop::start(
                        usb.as_ref(),
                        drivers.keyscan,
                        drivers.debounce,
                        mouse,
                        drivers.storage,
                        drivers.split,
                        drivers.backlight,
                        key_config,
                        hooks,
                    )
                    .await;
                }
                (None, None) => {
                    panic!("No USB or BLE driver is provided");
                }
            }
        },
    )
    .await;
}
