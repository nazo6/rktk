//! Program entrypoint.

use ekv::flash::Flash;
use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time::{Duration, Timer};
use report::ReportChannel;

use crate::{
    config::static_config::CONFIG,
    interface::{
        backlight::BacklightDriver,
        ble::BleDriver,
        display::{DisplayDriver, DummyDisplayDriver},
        double_tap::DoubleTapResetDriver,
        keyscan::KeyscanDriver,
        mouse::MouseDriver,
        split::SplitDriver,
        usb::UsbDriver,
        DriverBuilder,
    },
    keycode::Layer,
};

mod backlight;
pub mod display;
mod no_split;
mod report;
mod split;

/// All drivers required to run the keyboard.
///
/// Only the `key_scanner` and `usb` drivers are required.
/// For other drivers, if the value is None, it will be handled appropriately.
pub struct Drivers<
    // required drivers
    KeyScan: KeyscanDriver,
    // builder drivers
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    // optional drivers
    Backlight: BacklightDriver,
    Usb: UsbDriver,
    Split: SplitDriver,
    Ble: BleDriver,
    DoubleTapReset: DoubleTapResetDriver,
    EkvFlash: Flash + 'static,
    // optional drivers of builder
    Mouse: MouseDriver,
    Display: DisplayDriver,
> {
    pub double_tap_reset: Option<DoubleTapReset>,
    pub key_scanner: KeyScan,
    pub backlight: Option<Backlight>,
    pub usb: Option<Usb>,
    pub ble: Option<Ble>,
    pub split: Option<Split>,
    pub storage: Option<&'static ekv::Database<EkvFlash, CriticalSectionRawMutex>>,

    pub mouse_builder: Option<MouseBuilder>,
    pub display_builder: Option<DisplayBuilder>,
}

/// Receives the [`Drivers`] and executes the main process of the keyboard.
/// This function should not be called more than once.
///
/// NOTE: For optimal boot time and proper operation of the Double Tap driver, do not do any heavy processing before executing this function.
/// Driver authors should use the init method defined in each driver's trait to perform initialization
/// instead of the associated functions such as new that are performed on the keyboard crate side.
///
/// TODO: To avoid using both `new` and `init` methods, receive builder instead of driver.
pub async fn start<
    // required drivers
    KeyScan: KeyscanDriver,
    // builder drivers
    MouseBuilder: DriverBuilder<Output = Mouse>,
    DisplayBuilder: DriverBuilder<Output = Display>,
    // optional drivers
    Backlight: BacklightDriver,
    Usb: UsbDriver,
    Split: SplitDriver,
    Ble: BleDriver,
    DoubleTapReset: DoubleTapResetDriver,
    EkvFlash: Flash + 'static,
    // optional drivers of builder
    Mouse: MouseDriver,
    Display: DisplayDriver,
>(
    mut drivers: Drivers<
        KeyScan,
        MouseBuilder,
        DisplayBuilder,
        Backlight,
        Usb,
        Split,
        Ble,
        DoubleTapReset,
        EkvFlash,
        Mouse,
        Display,
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

            let host_connected = match (&mut drivers.ble, &mut drivers.usb) {
                (Some(ble), _) => {
                    let _ = ble.wait_ready().await;
                    true
                }
                (_, Some(usb)) => {
                    match select(
                        usb.wait_ready(),
                        Timer::after_millis(CONFIG.split_usb_timeout),
                    )
                    .await
                    {
                        Either::First(_) => true,
                        Either::Second(_) => false,
                    }
                }
                _ => false,
            };

            let report_chan = ReportChannel::new();
            let report_sender = report_chan.sender();
            let report_receiver = report_chan.receiver();

            join(
                async {
                    if let Some(split) = drivers.split {
                        split::start(
                            report_sender,
                            drivers.key_scanner,
                            mouse,
                            split,
                            drivers.backlight,
                            keymap,
                            host_connected,
                        )
                        .await;
                    } else {
                        no_split::start(
                            report_sender,
                            keymap,
                            drivers.key_scanner,
                            mouse,
                            drivers.backlight,
                        )
                        .await;
                    }
                },
                async {
                    report::start_report_task(report_receiver, drivers.usb, drivers.ble).await;
                },
            )
            .await;
        },
    )
    .await;
}

pub enum DummyDisplayDriverBuilder {}
impl DriverBuilder for DummyDisplayDriverBuilder {
    type Output = DummyDisplayDriver;

    type Error = ();

    async fn build(self) -> Result<Self::Output, Self::Error> {
        unimplemented!()
    }
}
