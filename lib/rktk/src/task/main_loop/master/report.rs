use embassy_futures::select::{select4, Either4};
use embassy_time::{Duration, Instant};
use rktk_keymanager::interface::{report::StateReport, state::input_event::InputEvent, Output};
use rktk_keymanager::state::hooks::Hooks as KeymanagerHooks;
use rktk_log::{debug, helper::Debug2Format};

use crate::{
    config::{constant::RKTK_CONFIG, storage::StorageConfigManager},
    drivers::interface::{
        ble::BleDriver, reporter::ReporterDriver, storage::StorageDriver, system::SystemDriver,
        usb::UsbDriver,
    },
    hooks::interface::MasterHooks,
    task::channels::report::{
        ENCODER_EVENT_REPORT_CHANNEL, KEYBOARD_EVENT_REPORT_CHANNEL, MOUSE_EVENT_REPORT_CHANNEL,
    },
    utils::display_state,
};

use super::SharedState;

pub async fn report_task<
    System: SystemDriver,
    S: StorageDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    MH: MasterHooks,
    KH: KeymanagerHooks,
>(
    system: &System,
    state: &SharedState<KH>,
    config_store: &Option<StorageConfigManager<S>>,
    ble: &Option<Ble>,
    usb: &Option<Usb>,
    mut master_hooks: MH,
) {
    debug!("report task start");

    let mut prev_update_time = embassy_time::Instant::now();
    let mut current_output = state.lock().await.get_config().initial_output;
    let mut prev_report = None;
    let mut display_off = DisplayOffController::new();

    loop {
        let event = match select4(
            MOUSE_EVENT_REPORT_CHANNEL.ready_to_receive(),
            KEYBOARD_EVENT_REPORT_CHANNEL.receive(),
            ENCODER_EVENT_REPORT_CHANNEL.receive(),
            read_keyboard_report(usb, ble, current_output),
        )
        .await
        {
            Either4::First(_) => {
                let mut mouse_move: (i8, i8) = (0, 0);
                while let Ok((x, y)) = MOUSE_EVENT_REPORT_CHANNEL.try_receive() {
                    mouse_move.0 += x;
                    mouse_move.1 += y;
                }

                if !master_hooks.on_mouse_event(&mut mouse_move).await {
                    continue;
                }

                InputEvent::Mouse(mouse_move)
            }
            Either4::Second(mut event) => {
                if !master_hooks.on_keyboard_event(&mut event).await {
                    continue;
                }

                InputEvent::Key(event)
            }
            Either4::Third((mut id, mut dir)) => {
                if !master_hooks.on_encoder_event(&mut id, &mut dir).await {
                    continue;
                }

                InputEvent::Encoder((id, dir))
            }
            Either4::Fourth(report) => {
                if let Ok(report) = report {
                    crate::utils::display_state!(NumLock, (report & 1) == 1);
                    crate::utils::display_state!(CapsLock, (report & 2) == 2);
                }

                continue;
            }
        };

        let mut state_report = state
            .lock()
            .await
            .update(event, prev_update_time.elapsed().into());

        prev_update_time = embassy_time::Instant::now();
        if prev_report == Some(state_report.clone()) {
            display_off.update(false);
            continue;
        }

        prev_report = Some(state_report.clone());

        if !master_hooks
            .on_state_update(&mut state_report, usb, ble)
            .await
        {
            display_off.update(false);
            continue;
        }

        crate::utils::display_state!(HighestLayer, state_report.highest_layer);

        if state_report.transparent_report.bootloader {
            system.reset_to_bootloader();
        }

        if state_report.transparent_report.flash_clear {
            if let Some(ref storage) = config_store {
                match storage.storage.format().await {
                    Ok(_) => {
                        rktk_log::info!("Storage formatted by report");
                        crate::print!("Storage formatted")
                    }
                    Err(e) => {
                        rktk_log::error!("Failed to format storage: {:?}", Debug2Format(&e));
                        crate::print!("Failed to format storage: {:?}", Debug2Format(&e));
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

        current_output = state_report.transparent_report.output;
        let reported = match state_report.transparent_report.output {
            Output::Usb => {
                crate::utils::display_state!(Output, Output::Usb);
                if let Some(usb) = &usb {
                    send_report(usb, state_report).await
                } else {
                    false
                }
            }
            Output::Ble => {
                crate::utils::display_state!(Output, Output::Ble);
                if let Some(ble) = &ble {
                    send_report(ble, state_report).await
                } else {
                    false
                }
            }
        };
        display_off.update(reported);
    }
}

async fn send_report(reporter: &impl ReporterDriver, state_report: StateReport) -> bool {
    let mut reported = false;
    if let Some(report) = state_report.keyboard_report {
        reported = true;
        let report = if let Ok(true) = reporter.wakeup() {
            usbd_hid::descriptor::KeyboardReport::default()
        } else {
            report
        };

        if let Err(e) = reporter.try_send_keyboard_report(report) {
            rktk_log::warn!("Failed to send keyboard report: {:?}", Debug2Format(&e));
        }
    }
    if let Some(report) = state_report.mouse_report {
        reported = true;
        crate::utils::display_state!(MouseMove, (report.x, report.y));
        if let Err(e) = reporter.try_send_mouse_report(report) {
            rktk_log::warn!("Failed to send mouse report: {:?}", Debug2Format(&e));
        }
    }
    if let Some(report) = state_report.media_keyboard_report {
        reported = true;
        if let Err(e) = reporter.try_send_media_keyboard_report(report) {
            rktk_log::warn!(
                "Failed to send media keyboard report: {:?}",
                Debug2Format(&e)
            );
        }
    }

    reported
}

async fn read_keyboard_report<USB: UsbDriver, BLE: BleDriver>(
    usb: &Option<USB>,
    ble: &Option<BLE>,
    output: Output,
) -> Result<u8, ()> {
    let report = match output {
        Output::Usb => {
            if let Some(usb) = usb {
                usb.read_keyboard_report().await.map_err(|e| {
                    rktk_log::warn!("Failed to read keyboard report: {:?}", Debug2Format(&e));
                })?
            } else {
                let _: () = core::future::pending().await;
                unreachable!()
            }
        }
        Output::Ble => {
            if let Some(usb) = ble {
                usb.read_keyboard_report().await.map_err(|e| {
                    rktk_log::warn!("Failed to read keyboard report: {:?}", Debug2Format(&e));
                })?
            } else {
                let _: () = core::future::pending().await;
                unreachable!()
            }
        }
    };

    Ok(report)
}

struct DisplayOffController {
    display_off: bool,
    latest_report_time: Instant,
}

impl DisplayOffController {
    fn new() -> Self {
        Self {
            display_off: false,
            latest_report_time: Instant::now(),
        }
    }
    fn update(&mut self, reported: bool) {
        if reported {
            self.latest_report_time = Instant::now();
        }

        if Instant::now() - self.latest_report_time
            > Duration::from_millis(RKTK_CONFIG.display_timeout)
        {
            if !self.display_off {
                display_state!(On, false);
                self.display_off = true;
            }
        } else if self.display_off {
            display_state!(On, true);
            self.display_off = false;
        }
    }
}
