//! Program entrypoint.

use ekv::flash::Flash;
use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Timer};

use crate::{
    config::static_config::CONFIG,
    interface::{
        backlight::BacklightDriver, ble::BleDriver, display::DisplayDriver,
        double_tap::DoubleTapResetDriver, keyscan::KeyscanDriver, mouse::MouseDriver,
        reporter::DummyReporterDriver, split::SplitDriver, usb::UsbDriver, DriverBuilder,
    },
    Layer,
};

mod backlight;
pub mod display;
mod main_loop;

/// All drivers required to run the keyboard.
///
/// Only the `key_scanner` and `usb` drivers are required.
/// For other drivers, if the value is None, it will be handled appropriately.
pub struct Drivers<
    // required drivers
    KeyScan: KeyscanDriver,
    Split: SplitDriver,
    // builder drivers
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilder<Output = Usb>,
    // optional drivers
    Backlight: BacklightDriver,
    Ble: BleDriver,
    DoubleTapReset: DoubleTapResetDriver,
    EkvFlash: Flash + 'static,
    // optional drivers of builder
    Mouse: MouseDriver,
    Display: DisplayDriver,
    Usb: UsbDriver,
> {
    pub double_tap_reset: Option<DoubleTapReset>,
    pub key_scanner: KeyScan,
    pub split: Split,
    pub backlight: Option<Backlight>,
    pub ble: Option<Ble>,
    pub storage: Option<&'static ekv::Database<EkvFlash, CriticalSectionRawMutex>>,

    pub usb_builder: Option<UsbBuilder>,
    pub mouse_builder: Option<MouseBuilder>,
    pub display_builder: Option<DisplayBuilder>,
}

/// Receives the [`Drivers`] and executes the main process of the keyboard.
/// This function must not be called more than once.
pub async fn start<
    KeyScan: KeyscanDriver,
    Split: SplitDriver,
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    UsbBuilder: DriverBuilder<Output = Usb>,
    Backlight: BacklightDriver,
    Ble: BleDriver,
    DoubleTapReset: DoubleTapResetDriver,
    EkvFlash: Flash + 'static,
    Mouse: MouseDriver,
    Display: DisplayDriver,
    Usb: UsbDriver,
>(
    mut drivers: Drivers<
        KeyScan,
        Split,
        MouseBuilder,
        DisplayBuilder,
        UsbBuilder,
        Backlight,
        Ble,
        DoubleTapReset,
        EkvFlash,
        Mouse,
        Display,
        Usb,
    >,
    keymap: [Layer; CONFIG.layer_count],
) {
    if let Some(dtr) = &mut drivers.double_tap_reset {
        dtr.execute(Duration::from_millis(CONFIG.double_tap_threshold))
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
                if let Ok(mut mouse) = mouse_builder.build().await {
                    let _ = mouse.set_cpi(600).await;
                    Some(mouse)
                } else {
                    None
                }
            } else {
                None
            };

            crate::utils::display_state!(MouseAvailable, mouse.is_some());

            match (drivers.ble, drivers.usb_builder) {
                (Some(ble), _) => {
                    main_loop::start(
                        Some(&ble),
                        drivers.key_scanner,
                        mouse,
                        drivers.split,
                        drivers.backlight,
                        keymap,
                    )
                    .await;
                }
                (_, Some(usb_builder)) => {
                    match select(
                        usb_builder.build(),
                        Timer::after(Duration::from_millis(CONFIG.split_usb_timeout)),
                    )
                    .await
                    {
                        Either::First(usb) => {
                            let Ok(usb) = usb else {
                                panic!("Failed to build USB");
                            };

                            main_loop::start(
                                Some(&usb),
                                drivers.key_scanner,
                                mouse,
                                drivers.split,
                                drivers.backlight,
                                keymap,
                            )
                            .await;
                        }
                        Either::Second(_) => {
                            main_loop::start(
                                Option::<DummyReporterDriver>::None.as_ref(),
                                drivers.key_scanner,
                                mouse,
                                drivers.split,
                                drivers.backlight,
                                keymap,
                            )
                            .await;
                        }
                    }
                }
                (None, None) => {
                    panic!("No USB or BLE driver is provided");
                }
            }
        },
    )
    .await;
}
