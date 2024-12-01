use embassy_futures::select::{select3, Either3};
use rktk_keymanager::state::{config::Output, StateReport};

use crate::{
    config::storage_config::StorageConfigManager,
    drivers::interface::{
        ble::BleDriver, reporter::ReporterDriver, storage::StorageDriver, usb::UsbDriver,
    },
    hooks::interface::MasterHooks,
    task::channels::report::{
        ENCODER_EVENT_REPORT_CHANNEL, KEYBOARD_EVENT_REPORT_CHANNEL, MOUSE_EVENT_REPORT_CHANNEL,
    },
};

use super::SharedState;

pub async fn report_task<S: StorageDriver, Ble: BleDriver, Usb: UsbDriver, MH: MasterHooks>(
    state: &SharedState,
    config_store: &Option<StorageConfigManager<S>>,
    ble: &Option<Ble>,
    usb: &Option<Usb>,
    mut master_hooks: MH,
) {
    let mut prev_report_time = embassy_time::Instant::now();
    loop {
        let mut state_report = match select3(
            MOUSE_EVENT_REPORT_CHANNEL.ready_to_receive(),
            KEYBOARD_EVENT_REPORT_CHANNEL.ready_to_receive(),
            ENCODER_EVENT_REPORT_CHANNEL.ready_to_receive(),
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

                state.lock().await.update(
                    &mut [],
                    mouse_move,
                    &[],
                    (embassy_time::Instant::now() - prev_report_time).into(),
                )
            }
            Either3::Second(_) => {
                let mut events = heapless::Vec::<_, 5>::new();
                while let Ok(mut ev) = KEYBOARD_EVENT_REPORT_CHANNEL.try_receive() {
                    if !master_hooks.on_keyboard_event(&mut ev).await {
                        continue;
                    }

                    events.push(ev).ok();
                    if events.len() >= events.capacity() {
                        break;
                    }
                }

                if events.is_empty() {
                    continue;
                }

                state.lock().await.update(
                    &mut events,
                    (0, 0),
                    &[],
                    (embassy_time::Instant::now() - prev_report_time).into(),
                )
            }
            Either3::Third(_) => {
                let (mut id, mut dir) = ENCODER_EVENT_REPORT_CHANNEL.receive().await;
                if !master_hooks.on_encoder_event(&mut id, &mut dir).await {
                    continue;
                }
                state.lock().await.update(
                    &mut [],
                    (0, 0),
                    &[(id, dir)],
                    (embassy_time::Instant::now() - prev_report_time).into(),
                )
            }
        };

        master_hooks.on_state_update(&mut state_report).await;

        crate::utils::display_state!(HighestLayer, state_report.highest_layer);

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
                ble.clear_bond_data().await;
            }
        }

        match state_report.transparent_report.output {
            Output::Usb => {
                crate::utils::display_state!(Output, Output::Usb);
                if let Some(usb) = &usb {
                    send_report(usb, state_report);
                }
            }
            Output::Ble => {
                crate::utils::display_state!(Output, Output::Ble);
                if let Some(ble) = &ble {
                    send_report(ble, state_report);
                }
            }
        }

        prev_report_time = embassy_time::Instant::now();
    }
}

fn send_report(reporter: &impl ReporterDriver, state_report: StateReport) {
    if let Some(report) = state_report.keyboard_report {
        if let Err(e) = reporter.try_send_keyboard_report(report) {
            log::warn!("Failed to send keyboard report: {:?}", e);
        }
        let _ = reporter.wakeup();
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
