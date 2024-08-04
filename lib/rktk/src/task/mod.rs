//! Program entrypoint.

use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_time::{Duration, Timer};
use report::ReportChannel;

use crate::{
    config::static_config::CONFIG,
    interface::{
        backlight::BacklightDriver, ble::BleDriver, display::DisplayDriver,
        double_tap::DoubleTapResetDriver, keyscan::KeyscanDriver, mouse::MouseDriver,
        split::SplitDriver, usb::UsbDriver,
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
///
/// TODO: Add bluetooth driver and make usb optional.
pub struct Drivers<
    DTR: DoubleTapResetDriver,
    KS: KeyscanDriver,
    M: MouseDriver,
    USB: UsbDriver,
    D: DisplayDriver,
    SP: SplitDriver,
    BL: BacklightDriver,
    BT: BleDriver,
> {
    pub double_tap_reset: Option<DTR>,

    pub key_scanner: KS,
    pub mouse: Option<M>,

    pub display: Option<D>,

    pub split: Option<SP>,
    pub backlight: Option<BL>,

    pub usb: Option<USB>,
    pub ble: Option<BT>,
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
    DTR: DoubleTapResetDriver,
    KS: KeyscanDriver,
    M: MouseDriver,
    USB: UsbDriver,
    D: DisplayDriver,
    SP: SplitDriver,
    BL: BacklightDriver,
    BT: BleDriver,
>(
    mut drivers: Drivers<DTR, KS, M, USB, D, SP, BL, BT>,
    keymap: [Layer; CONFIG.layer_count],
) {
    if let Some(dtr) = &mut drivers.double_tap_reset {
        dtr.execute(Duration::from_millis(CONFIG.double_tap_threshold))
            .await;
    }

    join(
        async move {
            if let Some(display) = drivers.display {
                display::start(display).await;
            }
        },
        async {
            let mouse = if let Some(mut mouse) = drivers.mouse {
                if mouse.init().await.is_ok() {
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
