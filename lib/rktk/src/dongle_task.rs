use rktk_log::helper::Debug2Format;

use crate::{
    drivers::interface::{
        display::DisplayDriver,
        dongle::{DongleData, DongleDriver as _, DongleDriverBuilder},
        reporter::ReporterDriver as _,
        usb::UsbReporterDriverBuilder,
    },
    hooks::interface::dongle::DongleHooks,
    task::display,
    utils::sjoin,
};

/// Runs dongle with the given drivers.
pub async fn start_dongle<D: display::DisplayConfig + 'static>(
    usb: impl UsbReporterDriverBuilder,
    dongle: impl DongleDriverBuilder,
    mut hooks: impl DongleHooks,
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
