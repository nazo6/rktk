use crate::{
    config::static_config::{KEYBOARD, RKTK_CONFIG},
    interface::{
        backlight::BacklightDriver,
        debounce::DebounceDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        reporter::ReporterDriver,
        split::{MasterToSlave, SlaveToMaster, SplitDriver},
        storage::StorageDriver,
    },
    KeyConfig,
};
use embassy_futures::join::join;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};

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
    'a,
    KS: KeyscanDriver,
    DB: DebounceDriver,
    M: MouseDriver,
    SP: SplitDriver,
    BL: BacklightDriver,
    R: ReporterDriver,
    S: StorageDriver,
    MainHooks: crate::hooks::MainHooks,
    BacklightHooks: crate::hooks::BacklightHooks,
>(
    reporter: Option<&R>,
    mut keyscan: KS,
    debounce: DB,
    mut mouse: Option<M>,
    mut storage: Option<S>,
    mut split: SP,
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
            let _ = split.init().await;

            hooks
                .main
                .on_init(
                    hand,
                    &mut keyscan,
                    mouse.as_mut(),
                    reporter,
                    storage.as_mut(),
                )
                .await;

            let is_master = reporter.is_some();

            crate::utils::display_state!(Master, Some(is_master));

            let s2m_chan: S2mChannel = Channel::new();
            let s2m_tx = s2m_chan.sender();
            let s2m_rx = s2m_chan.receiver();

            let m2s_chan: M2sChannel = Channel::new();
            let m2s_tx = m2s_chan.sender();
            let m2s_rx = m2s_chan.receiver();

            if let Some(reporter) = reporter {
                join(
                    split_handler::start(split, s2m_tx, m2s_rx, is_master),
                    master::start(
                        m2s_tx, s2m_rx, reporter, keyscan, debounce, storage, mouse, key_config,
                        hand, hooks.main,
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
        },
    )
    .await;
}
