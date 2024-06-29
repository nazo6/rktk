use embassy_futures::join::join;
use embassy_time::Duration;

use crate::{
    config::DOUBLE_TAP_THRESHOLD,
    constant::LAYER_COUNT,
    interface::{
        display::Display, double_tap::DoubleTapReset, keyscan::Keyscan, mouse::Mouse,
        split::SplitDriver, usb::UsbDriver,
    },
    keycode::Layer,
};

pub mod display;
mod split;

pub const MIN_KB_SCAN_INTERVAL: Duration = Duration::from_millis(5);

pub struct Drivers<
    DTR: DoubleTapReset,
    KS: Keyscan,
    M: Mouse,
    USB: UsbDriver,
    D: Display,
    SP: SplitDriver,
> {
    pub key_scanner: KS,
    pub double_tap_reset: Option<DTR>,
    pub mouse: Option<M>,
    pub usb: USB,
    pub display: Option<D>,
    pub split: SP,
}

// Start main task.
// This task does all the processing for rktk.
//
// NOTE: For optimal boot time and proper operation of the Double Tap driver, do not do any heavy processing before executing this function.
// Driver authors should use the init method defined in each driver's trace to perform initialization
// instead of the associated functions such as new that are performed on the keyboard crate side.
pub async fn start<
    DTR: DoubleTapReset,
    KS: Keyscan,
    M: Mouse,
    USB: UsbDriver,
    D: Display,
    SP: SplitDriver,
>(
    mut drivers: Drivers<DTR, KS, M, USB, D, SP>,
    keymap: [Layer; LAYER_COUNT],
) {
    if let Some(dtr) = &mut drivers.double_tap_reset {
        dtr.execute(DOUBLE_TAP_THRESHOLD).await;
    }

    join(
        async move {
            if let Some(display) = drivers.display {
                display::start(display).await;
            }
        },
        async {
            let mouse_available = if let Some(mouse) = &mut drivers.mouse {
                let init_ok = mouse.init().await.is_ok();
                if mouse.init().await.is_ok() {
                    let _ = mouse.set_cpi(600).await;
                }
                init_ok
            } else {
                false
            };

            crate::utils::display_state!(MouseAvailable, mouse_available);

            split::start(
                keymap,
                drivers.key_scanner,
                drivers.mouse,
                drivers.split,
                drivers.usb,
            )
            .await;
        },
    )
    .await;
}
