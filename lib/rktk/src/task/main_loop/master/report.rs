use embassy_futures::select::{select3, Either3};
use rktk_keymanager::{
    config::Output,
    state::{Event, StateReport},
};

use crate::{
    drivers::interface::{
        ble::BleDriver, reporter::ReporterDriver, storage::StorageDriver, system::SystemDriver,
        usb::UsbDriver,
    },
    hooks::interface::MasterHooks,
    task::{
        channels::report::{
            ENCODER_EVENT_REPORT_CHANNEL, KEYBOARD_EVENT_REPORT_CHANNEL, MOUSE_EVENT_REPORT_CHANNEL,
        },
        module::storage_config::StorageConfigManager,
    },
};

use super::SharedState;

pub async fn report_task<
    System: SystemDriver,
    S: StorageDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    MH: MasterHooks,
>(
    system: &System,
    state: &SharedState,
    config_store: &Option<StorageConfigManager<S>>,
    ble: &Option<Ble>,
    usb: &Option<Usb>,
    mut master_hooks: MH,
) {
    let mut prev_update_time = embassy_time::Instant::now();
    loop {
        let event = match select3(
            MOUSE_EVENT_REPORT_CHANNEL.ready_to_receive(),
            KEYBOARD_EVENT_REPORT_CHANNEL.receive(),
            ENCODER_EVENT_REPORT_CHANNEL.receive(),
        )
        .await
        {
            Either3::First(_) => {
                let mut mouse_move: (i8, i8) = (0, 0);
                while let Ok((x, y)) = MOUSE_EVENT_REPORT_CHANNEL.try_receive() {
                    mouse_move.0 += x;
                    mouse_move.1 += y;
                }

                if !master_hooks.on_mouse_event(&mut mouse_move).await {
                    continue;
                }

                Event::Mouse(mouse_move)
            }
            Either3::Second(mut event) => {
                if !master_hooks.on_keyboard_event(&mut event).await {
                    continue;
                }

                Event::Key(event)
            }
            Either3::Third((mut id, mut dir)) => {
                if !master_hooks.on_encoder_event(&mut id, &mut dir).await {
                    continue;
                }

                Event::Encoder((id, dir))
            }
        };

        let mut state_report = state
            .lock()
            .await
            .update(event, prev_update_time.elapsed().into());
        prev_update_time = embassy_time::Instant::now();

        master_hooks.on_state_update(&mut state_report).await;

        crate::utils::display_state!(HighestLayer, state_report.highest_layer);

        if state_report.transparent_report.bootloader {
            system.reset_to_bootloader();
        }

        if state_report.transparent_report.flash_clear {
            if let Some(ref storage) = config_store {
                match storage.storage.format().await {
                    Ok(_) => {
                        log::info!("Storage formatted by report");
                        crate::print!("Storage formatted")
                    }
                    Err(e) => {
                        log::error!("Failed to format storage: {:?}", e);
                        crate::print!("Failed to format storage: {:?}", e)
                    }
                }
            }
        }

        if state_report.transparent_report.ble_bond_clear {
            if let Some(ble) = &ble {
                let _ = ble.clear_bond_data().await;
            }
        }

        if state_report.transparent_report.power_off {
            system.power_off().await;
        }

        match state_report.transparent_report.output {
            Output::Usb => {
                crate::utils::display_state!(Output, Output::Usb);
                if let Some(usb) = &usb {
                    send_report(usb, state_report).await;
                }
            }
            Output::Ble => {
                crate::utils::display_state!(Output, Output::Ble);
                if let Some(ble) = &ble {
                    send_report(ble, state_report).await;
                }
            }
        }
    }
}

async fn send_report(reporter: &impl ReporterDriver, state_report: StateReport) {
    if let Some(report) = state_report.keyboard_report {
        let report = if let Ok(true) = reporter.wakeup() {
            usbd_hid::descriptor::KeyboardReport::default()
        } else {
            report
        };

        if let Err(e) = reporter.try_send_keyboard_report(report) {
            log::warn!("Failed to send keyboard report: {:?}", e);
        }
    }
    if let Some(report) = state_report.mouse_report {
        crate::utils::display_state!(MouseMove, (report.x, report.y));
        if let Err(e) = reporter.try_send_mouse_report(report) {
            log::warn!("Failed to send mouse report: {:?}", e);
        }
    }
    if let Some(report) = state_report.media_keyboard_report {
        if let Err(e) = reporter.try_send_media_keyboard_report(report) {
            log::warn!("Failed to send media keyboard report: {:?}", e);
        }
    }
}
