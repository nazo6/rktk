use crate::{
    config::static_config::{KEYBOARD, RKTK_CONFIG},
    interface::{
        backlight::BacklightDriver,
        ble::BleDriver,
        debounce::DebounceDriver,
        encoder::EncoderDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster, SplitDriver},
        storage::StorageDriver,
        usb::UsbDriver,
    },
    KeyConfig,
};
use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::{Duration, Timer};

mod master;
mod slave;
mod split_handler;

type S2mChannel =
    Channel<CriticalSectionRawMutex, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;
type S2mRx<'a> =
    Receiver<'a, CriticalSectionRawMutex, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;
pub type S2mTx<'a> =
    Sender<'a, CriticalSectionRawMutex, SlaveToMaster, { RKTK_CONFIG.split_channel_size }>;

type M2sChannel =
    Channel<CriticalSectionRawMutex, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;
type M2sRx<'a> =
    Receiver<'a, CriticalSectionRawMutex, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;
pub type M2sTx<'a> =
    Sender<'a, CriticalSectionRawMutex, MasterToSlave, { RKTK_CONFIG.split_channel_size }>;

#[allow(clippy::too_many_arguments)]
pub async fn start<
    KS: KeyscanDriver,
    DB: DebounceDriver,
    EN: EncoderDriver,
    M: MouseDriver,
    SP: SplitDriver,
    BL: BacklightDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    S: StorageDriver,
    MainHooks: crate::hooks::MainHooks,
    BacklightHooks: crate::hooks::BacklightHooks,
>(
    ble: Option<Ble>,
    usb: Option<Usb>,
    mut keyscan: KS,
    debounce: Option<DB>,
    encoder: Option<EN>,
    mut mouse: Option<M>,
    mut storage: Option<S>,
    mut split: Option<SP>,
    backlight: Option<BL>,
    key_config: KeyConfig,
    mut hooks: crate::hooks::Hooks<MainHooks, BacklightHooks>,
) {
    let hand = keyscan.current_hand().await;
    crate::utils::display_state!(Hand, Some(hand));

    join(
        async {
            if let Some(backlight) = backlight {
                match hand {
                    Hand::Right => {
                        super::backlight::start::<{ KEYBOARD.right_led_count }>(
                            backlight,
                            hooks.backlight,
                        )
                        .await
                    }
                    Hand::Left => {
                        super::backlight::start::<{ KEYBOARD.left_led_count }>(
                            backlight,
                            hooks.backlight,
                        )
                        .await
                    }
                }
            }
        },
        async {
            if let Some(split) = &mut split {
                let _ = split.init().await;
            }

            let usb_available = if let Some(usb) = &usb {
                match select(
                    usb.wait_ready(),
                    Timer::after(Duration::from_millis(RKTK_CONFIG.split_usb_timeout)),
                )
                .await
                {
                    Either::First(_) => true,
                    Either::Second(_) => false,
                }
            } else {
                false
            };

            let is_master = split.is_none() || usb_available || ble.is_some();

            hooks
                .main
                .on_init(
                    hand,
                    &mut keyscan,
                    mouse.as_mut(),
                    // reporter,
                    storage.as_mut(),
                )
                .await;

            crate::utils::display_state!(Master, Some(is_master));

            let s2m_chan: S2mChannel = Channel::new();
            let s2m_tx = s2m_chan.sender();
            let s2m_rx = s2m_chan.receiver();

            let m2s_chan: M2sChannel = Channel::new();
            let m2s_tx = m2s_chan.sender();
            let m2s_rx = m2s_chan.receiver();

            if let Some(split) = split {
                if is_master {
                    join(
                        split_handler::start(split, s2m_tx, m2s_rx, is_master),
                        master::start(
                            m2s_tx, s2m_rx, ble, usb, keyscan, debounce, encoder, storage, mouse,
                            key_config, hand, hooks.main,
                        ),
                    )
                    .await;
                } else {
                    join(
                        split_handler::start(split, m2s_tx, s2m_rx, is_master),
                        slave::start(s2m_tx, m2s_rx, keyscan, debounce, mouse, hooks.main),
                    )
                    .await;
                }
            } else {
                master::start(
                    m2s_tx, s2m_rx, ble, usb, keyscan, debounce, encoder, storage, mouse,
                    key_config, hand, hooks.main,
                )
                .await;
            }
        },
    )
    .await;
}
