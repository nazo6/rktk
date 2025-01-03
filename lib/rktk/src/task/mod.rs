//! Program entrypoint.

use crate::{
    config::Config,
    drivers::{interface::system::SystemDriver, Drivers},
    hooks::interface::*,
    keymap_config::Keymap,
};
use embassy_futures::join::{join, join3};
use embassy_time::Duration;
use static_cell::StaticCell;

use crate::{
    drivers::interface::{
        ble::BleDriver, debounce::DebounceDriver, display::DisplayDriver, encoder::EncoderDriver,
        keyscan::KeyscanDriver, mouse::MouseDriver, rgb::RgbDriver, split::SplitDriver,
        storage::StorageDriver, usb::UsbDriver, BackgroundTask as _, DriverBuilder,
        DriverBuilderWithTask,
    },
    hooks::Hooks,
};

pub(crate) mod channels;
pub mod display;
pub(crate) mod main_loop;
mod module;

static CONFIG_STORE: StaticCell<Config> = StaticCell::new();

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
    DisplayBuilder: DriverBuilder<Output = Display>,
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
    keymap: Keymap,
    hooks: Hooks<CH, MH, SH, BH>,
    config: Config,
) {
    critical_section::with(|_| unsafe {
        let _ = log::set_logger_racy(&module::logger::RRP_LOGGER);
        log::set_max_level_racy(log::LevelFilter::Info);
    });

    log::info!(
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
        .double_reset_usb_boot(Duration::from_millis(config.rktk.double_tap_threshold))
        .await;

    let config: &'static Config = CONFIG_STORE.init(config);

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
                        let _ = mouse.set_cpi(config.rktk.default_cpi).await;
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
                        keymap,
                        hooks,
                        config,
                    )
                    .await;
                },
            )
            .await;
        },
    )
    .await;
}
