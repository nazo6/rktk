use embassy_futures::join::{join, join5};
use embassy_time::Timer;
use rktk_keymanager::{
    interface::Output,
    state::{hooks::Hooks as KeymanagerHooks, State},
};
use rktk_log::{info, warn};
use utils::{init_storage, load_state};

use crate::{
    config::{
        constant::{KEYBOARD, KM_CONFIG, RKTK_CONFIG},
        keymap::Keymap,
    },
    drivers::interface::{
        ble::BleDriver,
        debounce::DebounceDriver,
        encoder::EncoderDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        storage::StorageDriver,
        system::SystemDriver,
        usb::UsbDriver,
    },
    hooks::interface::MasterHooks,
    task::channels::{
        report::{ENCODER_EVENT_REPORT_CHANNEL, MOUSE_EVENT_REPORT_CHANNEL},
        split::{M2sTx, S2mRx},
    },
    utils::Mutex,
};

mod handle_keyboard;
mod handle_mouse;
mod handle_slave;
mod report;
#[cfg(feature = "rrp")]
mod rrp_server;
mod utils;

type ConfiguredState<H> = State<
    H,
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
    { KEYBOARD.encoder_count as usize },
    { KM_CONFIG.constant.oneshot_state_size },
    { KM_CONFIG.constant.tap_dance_max_definitions },
    { KM_CONFIG.constant.tap_dance_max_repeats },
    { KM_CONFIG.constant.combo_key_max_definitions },
    { KM_CONFIG.constant.combo_key_max_sources },
>;

type SharedState<H> = Mutex<ConfiguredState<H>>;

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
    Sys: SystemDriver,
    MH: MasterHooks,
    KH: KeymanagerHooks,
>(
    _m2s_tx: M2sTx<'a>,
    s2m_rx: S2mRx<'a>,
    system: &Sys,
    ble: Option<Ble>,
    usb: Option<Usb>,
    mut keyscan: KS,
    debounce: Option<DB>,
    mut encoder: Option<EN>,
    storage: Option<S>,
    mut mouse: Option<M>,
    key_config: Keymap,
    hand: Hand,
    mut master_hooks: MH,
    key_manager_hooks: KH,
) {
    let config_store = init_storage(storage).await;
    let state = load_state(&config_store, key_config, Output::Usb, key_manager_hooks).await;

    info!("Master side task start");

    master_hooks
        .on_master_init(&mut keyscan, mouse.as_mut())
        .await;

    join(
        join(
            report::report_task(system, &state, &config_store, &ble, &usb, master_hooks),
            join5(
                handle_slave::start(hand, s2m_rx),
                handle_keyboard::start(hand, keyscan, debounce),
                handle_mouse::start(mouse),
                async {
                    if let Some(encoder) = &mut encoder {
                        loop {
                            let (id, dir) = encoder.read_wait().await;
                            if ENCODER_EVENT_REPORT_CHANNEL.try_send((id, dir)).is_err() {
                                warn!("enc full");
                            }
                        }
                    }
                },
                async {
                    // this is dummy task to make time-dependent things work
                    loop {
                        Timer::after_millis(10).await;
                        if MOUSE_EVENT_REPORT_CHANNEL.try_send((0, 0)).is_err() {
                            warn!("mouse full");
                        }
                    }
                },
            ),
        ),
        async {
            #[cfg(feature = "rrp")]
            rrp_server::start(&usb, &ble, &state, &config_store).await;
        },
    )
    .await;
}
