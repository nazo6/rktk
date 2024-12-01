use embassy_futures::{
    join::{join, join5},
    select::{select3, Either3},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Timer;
use rktk_keymanager::state::{
    config::{KeyResolverConfig, MouseConfig, Output, StateConfig},
    EncoderDirection, KeyChangeEvent, State, StateReport,
};

use crate::{
    config::{
        static_config::{KEYBOARD, RKTK_CONFIG, SCAN_INTERVAL_KEYBOARD, SCAN_INTERVAL_MOUSE},
        storage_config::StorageConfigManager,
    },
    drivers::interface::{
        backlight::{BacklightCommand, BacklightMode},
        ble::BleDriver,
        debounce::DebounceDriver,
        encoder::EncoderDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        reporter::ReporterDriver,
        split::{MasterToSlave, SlaveToMaster},
        storage::StorageDriver,
        usb::UsbDriver,
    },
    hooks::MainHooks,
    task::backlight::BACKLIGHT_CTRL,
    utils::ThreadModeMutex,
    KeyConfig,
};

use super::{M2sTx, S2mRx};

mod rrp_server;

type ConfiguredState = State<
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
    { KEYBOARD.encoder_count as usize },
>;

/// TODO: Currently, split index is changed like below.
/// Splitted:
/// 0 1 2 3 4   4 3 2 1 0
/// ↓
/// Entire:
/// 0 1 2 3 4   5 6 7 8 9
///
/// I'm not sure this is a common practice.
fn resolve_entire_key_pos(ev: &mut KeyChangeEvent, hand: Hand) {
    if hand == Hand::Right {
        ev.col = KEYBOARD.cols - 1 - ev.col;
    }
}

fn handle_led(
    state_report: &StateReport,
    m2s_tx: M2sTx<'_>,
    latest_led: &mut Option<BacklightCommand>,
) {
    let led = match state_report.highest_layer {
        1 => BacklightCommand::Start(BacklightMode::SolidColor(0, 0, 1)),
        2 => BacklightCommand::Start(BacklightMode::SolidColor(1, 0, 0)),
        3 => BacklightCommand::Start(BacklightMode::SolidColor(0, 1, 0)),
        4 => BacklightCommand::Start(BacklightMode::SolidColor(1, 1, 0)),
        _ => BacklightCommand::Reset,
    };

    if let Some(latest_led) = &latest_led {
        if led != *latest_led {
            let _ = BACKLIGHT_CTRL.try_send(led.clone());
            let _ = m2s_tx.try_send(MasterToSlave::Backlight(led.clone()));
        }
    }

    *latest_led = Some(led);
}

#[allow(clippy::too_many_arguments)]
pub async fn start<
    'a,
    KS: KeyscanDriver,
    DB: DebounceDriver,
    EN: EncoderDriver,
    M: MouseDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    S: StorageDriver,
    MH: MainHooks,
>(
    m2s_tx: M2sTx<'a>,
    s2m_rx: S2mRx<'a>,
    mut ble: Option<Ble>,
    usb: Option<Usb>,
    mut keyscan: KS,
    mut debounce: Option<DB>,
    mut encoder: Option<EN>,
    storage: Option<S>,
    mut mouse: Option<M>,
    key_config: KeyConfig,
    hand: crate::drivers::interface::keyscan::Hand,
    mut hook: MH,
) {
    let mut config_storage = None;
    if let Some(s) = storage {
        let s = StorageConfigManager::new(s);

        match s.read_version().await {
            Ok(1) => {
                log::info!("Storage version matched");
                config_storage = Some(s);
            }
            Ok(i) => {
                log::warn!("Storage version matched");
                crate::print!("Storage version mismatch: {}", i);
            }
            Err(_e) => match s.write_version(1).await {
                Ok(_) => {
                    config_storage = Some(s);
                }
                Err(e) => {
                    log::error!("Storage to write version to storage: {:?}", e);
                    crate::print!("Failed to access storage: {:?}", e);
                }
            },
        }
    }

    let (state_config, keymap) = if let Some(storage) = &config_storage {
        let mut keymap = key_config.keymap;
        for l in 0..RKTK_CONFIG.layer_count {
            if let Ok(layer) = storage.read_keymap(l).await {
                keymap.layers[l as usize] = layer;
            }
        }

        let c = storage.read_state_config().await;

        (c.ok(), keymap)
    } else {
        (None, key_config.keymap)
    };

    let state_config = state_config.unwrap_or_else(|| StateConfig {
        mouse: MouseConfig {
            auto_mouse_layer: RKTK_CONFIG.default_auto_mouse_layer,
            auto_mouse_duration: RKTK_CONFIG.default_auto_mouse_duration,
            auto_mouse_threshold: RKTK_CONFIG.default_auto_mouse_threshold,
            scroll_divider_x: RKTK_CONFIG.default_scroll_divider_x,
            scroll_divider_y: RKTK_CONFIG.default_scroll_divider_y,
        },
        key_resolver: KeyResolverConfig {
            tap_threshold: RKTK_CONFIG.default_tap_threshold,
            tap_dance_threshold: RKTK_CONFIG.default_tap_dance_threshold,
            tap_dance: key_config.tap_dance.clone(),
        },
        initial_output: if usb.is_some() {
            Output::Usb
        } else {
            Output::Ble
        },
    });

    let state = ThreadModeMutex::new(State::new(keymap, state_config));

    log::info!("Master side task start");

    hook.on_master_init(&mut keyscan, mouse.as_mut(), &m2s_tx)
        .await;

    let mut latest_led: Option<BacklightCommand> = None;

    join(
        async {
            let mouse_move_ch: Channel<CriticalSectionRawMutex, (i8, i8), 10> = Channel::new();
            let mouse_move_ch_sender = mouse_move_ch.sender();
            let mouse_move_ch_receiver = mouse_move_ch.receiver();

            let key_event_ch: Channel<CriticalSectionRawMutex, KeyChangeEvent, 10> = Channel::new();
            let key_event_ch_sender = key_event_ch.sender();
            let key_event_ch_receiver = key_event_ch.receiver();

            let encoder_event_ch: Channel<CriticalSectionRawMutex, (u8, EncoderDirection), 10> =
                Channel::new();
            let encoder_event_ch_sender = encoder_event_ch.sender();
            let encoder_event_ch_receiver = encoder_event_ch.receiver();

            join(
                async {
                    let mut prev_report_time = embassy_time::Instant::now();
                    loop {
                        let state_report = match select3(
                            mouse_move_ch_receiver.ready_to_receive(),
                            key_event_ch_receiver.ready_to_receive(),
                            encoder_event_ch_receiver.ready_to_receive(),
                        )
                        .await
                        {
                            Either3::First(_) => {
                                let mut mouse_move: (i8, i8) = (0, 0);
                                while let Ok((x, y)) = mouse_move_ch_receiver.try_receive() {
                                    mouse_move.0 += x;
                                    mouse_move.1 += y;
                                }

                                state.lock().await.update(
                                    &mut [],
                                    mouse_move,
                                    &[],
                                    (embassy_time::Instant::now() - prev_report_time).into(),
                                )
                            }
                            Either3::Second(_) => {
                                let mut events = heapless::Vec::<_, 10>::new();
                                while let Ok(ev) = key_event_ch_receiver.try_receive() {
                                    events.push(ev).ok();
                                }
                                state.lock().await.update(
                                    &mut events,
                                    (0, 0),
                                    &[],
                                    (embassy_time::Instant::now() - prev_report_time).into(),
                                )
                            }
                            Either3::Third(_) => {
                                let (id, dir) = encoder_event_ch_receiver.receive().await;
                                state.lock().await.update(
                                    &mut [],
                                    (0, 0),
                                    &[(id, dir)],
                                    (embassy_time::Instant::now() - prev_report_time).into(),
                                )
                            }
                        };

                        handle_led(&state_report, m2s_tx, &mut latest_led);

                        crate::utils::display_state!(HighestLayer, state_report.highest_layer);

                        if state_report.transparent_report.flash_clear {
                            if let Some(ref storage) = config_storage {
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
                            if let Some(ble) = &mut ble {
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
                },
                join5(
                    // slave
                    async {
                        loop {
                            s2m_rx.ready_to_receive().await;
                            while let Ok(cmd_from_slave) = s2m_rx.try_receive() {
                                match cmd_from_slave {
                                    SlaveToMaster::Pressed(row, col) => {
                                        let mut ev = KeyChangeEvent {
                                            col,
                                            row,
                                            pressed: true,
                                        };
                                        resolve_entire_key_pos(&mut ev, hand);
                                        key_event_ch_sender.send(ev).await;
                                    }
                                    SlaveToMaster::Released(row, col) => {
                                        let mut ev = KeyChangeEvent {
                                            col,
                                            row,
                                            pressed: false,
                                        };
                                        resolve_entire_key_pos(&mut ev, hand);
                                        key_event_ch_sender.send(ev).await;
                                    }
                                    SlaveToMaster::Mouse { x, y } => {
                                        mouse_move_ch_sender.send((x, y)).await;
                                    }
                                    SlaveToMaster::Message(_) => {}
                                }
                            }
                        }
                    },
                    // key
                    async {
                        loop {
                            Timer::after(SCAN_INTERVAL_KEYBOARD).await;

                            let mut buf = heapless::Vec::<_, 32>::new();
                            keyscan
                                .scan(|event| {
                                    let _ = buf.push(event);
                                })
                                .await;
                            for mut event in buf {
                                if let Some(debounce) = &mut debounce {
                                    if debounce
                                        .should_ignore_event(&event, embassy_time::Instant::now())
                                    {
                                        return;
                                    }
                                }
                                resolve_entire_key_pos(&mut event, hand);

                                let _ = key_event_ch_sender.try_send(event);
                            }
                        }
                    },
                    // mouse
                    async {
                        if let Some(mouse) = &mut mouse {
                            let mut empty_sent = false;
                            loop {
                                Timer::after(SCAN_INTERVAL_MOUSE).await;

                                let mouse_move = match mouse.read().await {
                                    Ok(m) => m,
                                    Err(e) => {
                                        log::warn!("Failed to read mouse: {:?}", e);
                                        crate::print!("{:?}", e);
                                        continue;
                                    }
                                };

                                if mouse_move == (0, 0) && empty_sent {
                                    continue;
                                } else {
                                    let _ = mouse_move_ch_sender.try_send(mouse_move);
                                    empty_sent = mouse_move == (0, 0);
                                }
                            }
                        }
                    },
                    async {
                        if let Some(encoder) = &mut encoder {
                            loop {
                                let (id, dir) = encoder.read_wait().await;
                                let _ = encoder_event_ch_sender.try_send((id, dir));
                            }
                        }
                    },
                    async {
                        // this is dummy task to make time-dependent things work
                        loop {
                            Timer::after(SCAN_INTERVAL_KEYBOARD).await;
                            let _ = mouse_move_ch_sender.try_send((0, 0));
                        }
                    },
                ),
            )
            .await;
        },
        async {
            if let Some(usb) = &usb {
                let mut server = rktk_rrp::server::Server::<_, _, _, 512>::new(
                    rrp_server::ServerTransport::new(usb),
                    rrp_server::ServerTransport::new(usb),
                    rrp_server::Handlers {
                        state: &state,
                        storage: config_storage.as_ref(),
                    },
                );
                server.start().await;
            }
        },
    )
    .await;
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
