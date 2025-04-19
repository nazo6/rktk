use core::sync::atomic::Ordering;

use embassy_futures::select::{Either, Either4, select, select4};
use embassy_time::{Duration, Instant};
use rktk_keymanager::interface::state::output_event::EventType;
use rktk_keymanager::interface::state::{input_event::InputEvent, output_event::OutputEvent};
use rktk_keymanager::state::hid_report::Report;
use rktk_log::{debug, helper::Debug2Format};

use crate::config::keymap::prelude::RktkKeys;
use crate::task::channels::report::{MOUSE_CHANGE_SIGNAL, MOUSE_CHANGE_X, MOUSE_CHANGE_Y};
use crate::{
    config::{constant::RKTK_CONFIG, storage::StorageConfigManager},
    drivers::interface::{
        reporter::{Output, ReporterDriver},
        storage::StorageDriver,
        system::SystemDriver,
        usb::UsbReporterDriver,
        wireless::WirelessReporterDriver,
    },
    hooks::interface::MasterHooks,
    task::channels::report::{ENCODER_EVENT_REPORT_CHANNEL, KEYBOARD_EVENT_REPORT_CHANNEL},
    utils::display_state,
};

use super::SharedState;

pub async fn report_task<
    System: SystemDriver,
    S: StorageDriver,
    Ble: WirelessReporterDriver,
    Usb: UsbReporterDriver,
    MH: MasterHooks,
>(
    system: &System,
    state: &SharedState,
    config_store: &Option<StorageConfigManager<S>>,
    ble: &Option<Ble>,
    usb: &Option<Usb>,
    mut master_hooks: MH,
) {
    debug!("report task start");

    let mut prev_update_time = embassy_time::Instant::now();
    let mut current_output = if usb.is_some() {
        Output::Usb
    } else {
        Output::Ble
    };
    let mut display_off = DisplayOffController::new();

    loop {
        let event = match select4(
            MOUSE_CHANGE_SIGNAL.wait(),
            KEYBOARD_EVENT_REPORT_CHANNEL.receive(),
            ENCODER_EVENT_REPORT_CHANNEL.receive(),
            select(
                read_keyboard_report(usb, ble, current_output),
                embassy_time::Timer::after_millis(RKTK_CONFIG.state_update_interval),
            ),
        )
        .await
        {
            Either4::First(_) => {
                let mut mouse_move: (i8, i8) = (
                    MOUSE_CHANGE_X.swap(0, Ordering::Acquire),
                    MOUSE_CHANGE_Y.swap(0, Ordering::Acquire),
                );

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
            Either4::Fourth(r) => match r {
                Either::First(report) => {
                    if let Ok(report) = report {
                        crate::utils::display_state!(NumLock, (report & 1) == 1);
                        crate::utils::display_state!(CapsLock, (report & 2) == 2);
                    }
                    continue;
                }
                Either::Second(_) => InputEvent::None,
            },
        };

        struct RktkKeyState {
            bootloader: bool,
            ble_bond_clear: bool,
            flash_clear: bool,
            power_off: bool,
        }
        let mut rktk_key_state = RktkKeyState {
            bootloader: false,
            ble_bond_clear: false,
            flash_clear: false,
            power_off: false,
        };

        let (mut state_report, layer_active) = {
            let mut s = state.lock().await;

            (
                s.update_with_cb(event, prev_update_time.elapsed().into(), |ev| {
                    if let OutputEvent::Custom(1, (id, et)) = ev {
                        if et == EventType::Pressed {
                            if let Ok(k) = TryInto::<RktkKeys>::try_into(id) {
                                match k {
                                    RktkKeys::FlashClear => rktk_key_state.flash_clear = true,
                                    RktkKeys::OutputBle => current_output = Output::Ble,
                                    RktkKeys::OutputUsb => current_output = Output::Usb,
                                    RktkKeys::BleBondClear => rktk_key_state.ble_bond_clear = true,
                                    RktkKeys::Bootloader => rktk_key_state.bootloader = true,
                                    RktkKeys::PowerOff => rktk_key_state.power_off = true,
                                }
                            }
                        }
                    } else {
                        master_hooks.on_keymanager_event(ev);
                    }
                }),
                *s.inner().get_layer_active(),
            )
        };

        prev_update_time = embassy_time::Instant::now();

        if !master_hooks
            .on_state_update(&mut state_report, usb, ble)
            .await
        {
            display_off.update(false);
            continue;
        }

        crate::utils::display_state!(LayerState, layer_active);

        if rktk_key_state.bootloader {
            system.reset_to_bootloader();
        }
        if rktk_key_state.flash_clear {
            if let Some(storage) = config_store.as_ref() {
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

        if rktk_key_state.ble_bond_clear {
            if let Some(ble) = &ble {
                let _ = ble.clear_bond_data().await;
            }
        }

        if rktk_key_state.power_off {
            system.power_off().await;
        }

        let reported = match current_output {
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

async fn send_report(reporter: &impl ReporterDriver, state_report: Report) -> bool {
    let mut reported = false;
    if let Some(report) = state_report.keyboard_report {
        reported = true;

        // Don't send wakeup signal if the report is empty
        let report = if report == usbd_hid::descriptor::KeyboardReport::default() {
            report
        } else if let Ok(true) = reporter.wakeup() {
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

async fn read_keyboard_report<USB: UsbReporterDriver, BLE: WirelessReporterDriver>(
    usb: &Option<USB>,
    ble: &Option<BLE>,
    output: Output,
) -> Result<u8, ()> {
    let report = match output {
        Output::Usb => {
            if let Some(usb) = usb {
                usb.recv_keyboard_report().await.map_err(|e| {
                    rktk_log::warn!("Failed to read keyboard report: {:?}", Debug2Format(&e));
                })?
            } else {
                let _: () = core::future::pending().await;
                unreachable!()
            }
        }
        Output::Ble => {
            if let Some(usb) = ble {
                usb.recv_keyboard_report().await.map_err(|e| {
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
