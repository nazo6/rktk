use embassy_futures::join::{join, join5};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Timer;
use rktk_keymanager::state::{config::Output, EncoderDirection, KeyChangeEvent, State};
use utils::{init_storage, load_state};

use crate::{
    config::static_config::{KEYBOARD, RKTK_CONFIG},
    drivers::interface::{
        ble::BleDriver,
        debounce::DebounceDriver,
        encoder::EncoderDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        storage::StorageDriver,
        usb::UsbDriver,
    },
    hooks::MainHooks,
    utils::ThreadModeMutex,
    KeyConfig,
};

use super::{M2sTx, S2mRx};

mod handle_keyboard;
mod handle_mouse;
mod handle_slave;
mod report;
mod rrp_server;
mod utils;

type ConfiguredState = State<
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
    { KEYBOARD.encoder_count as usize },
>;

type SharedState = ThreadModeMutex<ConfiguredState>;

static MOUSE_EVENT_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, (i8, i8), 5> = Channel::new();
static KEYBOARD_EVENT_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, KeyChangeEvent, 5> =
    Channel::new();
static ENCODER_EVENT_REPORT_CHANNEL: Channel<CriticalSectionRawMutex, (u8, EncoderDirection), 5> =
    Channel::new();

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
    ble: Option<Ble>,
    usb: Option<Usb>,
    mut keyscan: KS,
    debounce: Option<DB>,
    mut encoder: Option<EN>,
    storage: Option<S>,
    mut mouse: Option<M>,
    key_config: KeyConfig,
    hand: Hand,
    mut hook: MH,
) {
    let config_store = init_storage(storage).await;
    let state = load_state(&config_store, key_config, Output::Usb).await;

    log::info!("Master side task start");

    hook.on_master_init(&mut keyscan, mouse.as_mut(), &m2s_tx)
        .await;

    join(
        join(
            report::report_task(&state, &config_store, &ble, &usb),
            join5(
                handle_slave::start(hand, s2m_rx),
                handle_keyboard::start(hand, keyscan, debounce),
                handle_mouse::start(mouse),
                async {
                    if let Some(encoder) = &mut encoder {
                        loop {
                            let (id, dir) = encoder.read_wait().await;
                            let _ = ENCODER_EVENT_REPORT_CHANNEL.try_send((id, dir));
                        }
                    }
                },
                async {
                    // this is dummy task to make time-dependent things work
                    loop {
                        Timer::after_millis(10).await;
                        let _ = MOUSE_EVENT_REPORT_CHANNEL.try_send((0, 0));
                    }
                },
            ),
        ),
        rrp_server::start(&usb, &ble, &state, &config_store),
    )
    .await;
}
