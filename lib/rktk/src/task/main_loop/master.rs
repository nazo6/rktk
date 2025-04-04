use embassy_futures::join::{join, join4};
use rktk_keymanager::state::hid_report::HidReportState;
use rktk_log::{info, warn};
use utils::{init_storage, load_state};

use crate::{
    config::{
        constant::{KEYBOARD, KM_CONFIG, RKTK_CONFIG},
        keymap::Keymap,
    },
    drivers::interface::{
        debounce::DebounceDriver, encoder::EncoderDriver, keyscan::KeyscanDriver,
        mouse::MouseDriver, storage::StorageDriver, system::SystemDriver, usb::UsbReporterDriver,
        wireless::WirelessReporterDriver,
    },
    hooks::interface::MasterHooks,
    interface::Hand,
    task::channels::{
        report::ENCODER_EVENT_REPORT_CHANNEL,
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

type ConfiguredState = HidReportState<
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

type SharedState = Mutex<ConfiguredState>;

#[allow(clippy::too_many_arguments)]
pub async fn start<'a, MH: MasterHooks>(
    _m2s_tx: M2sTx<'a>,
    s2m_rx: S2mRx<'a>,
    system: &impl SystemDriver,
    ble: Option<impl WirelessReporterDriver>,
    usb: Option<impl UsbReporterDriver>,
    mut keyscan: impl KeyscanDriver,
    debounce: &mut Option<impl DebounceDriver>,
    mut encoder: Option<impl EncoderDriver>,
    storage: Option<impl StorageDriver>,
    mut mouse: Option<impl MouseDriver>,
    key_config: &Keymap,
    hand: Hand,
    mut master_hooks: MH,
) {
    let config_store = init_storage(storage).await;
    let state = load_state(&config_store, key_config).await;

    info!("Master side task start");

    master_hooks
        .on_master_init(&mut keyscan, mouse.as_mut())
        .await;

    join(
        join(
            report::report_task(system, &state, &config_store, &ble, &usb, master_hooks),
            join4(
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
            ),
        ),
        async {
            #[cfg(feature = "rrp")]
            rrp_server::start(&usb, &ble, &state, &config_store).await;
        },
    )
    .await;
}
