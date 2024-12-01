use crate::{
    config::static_config::{KEYBOARD, RKTK_CONFIG},
    drivers::interface::{
        backlight::BacklightDriver,
        ble::BleDriver,
        debounce::DebounceDriver,
        encoder::EncoderDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        split::SplitDriver,
        storage::StorageDriver,
        usb::UsbDriver,
    },
    hooks::{
        interface::{BacklightHooks, CommonHooks, MasterHooks, SlaveHooks},
        Hooks,
    },
    keymap_config::KeyConfig,
    task::channels::split::{M2S_CHANNEL, S2M_CHANNEL},
};
use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_time::{Duration, Timer};

mod master;
mod slave;
mod split_handler;

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
    CH: CommonHooks,
    MH: MasterHooks,
    SH: SlaveHooks,
    BH: BacklightHooks,
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
    mut hooks: Hooks<CH, MH, SH, BH>,
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
                .common
                .on_init(hand, &mut keyscan, mouse.as_mut(), storage.as_mut())
                .await;

            crate::utils::display_state!(Master, Some(is_master));

            let s2m_tx = S2M_CHANNEL.sender();
            let s2m_rx = S2M_CHANNEL.receiver();

            let m2s_tx = M2S_CHANNEL.sender();
            let m2s_rx = M2S_CHANNEL.receiver();

            if let Some(split) = split {
                if is_master {
                    join(
                        split_handler::start(split, s2m_tx, m2s_rx, is_master),
                        master::start(
                            m2s_tx,
                            s2m_rx,
                            ble,
                            usb,
                            keyscan,
                            debounce,
                            encoder,
                            storage,
                            mouse,
                            key_config,
                            hand,
                            hooks.master,
                        ),
                    )
                    .await;
                } else {
                    join(
                        split_handler::start(split, m2s_tx, s2m_rx, is_master),
                        slave::start(s2m_tx, m2s_rx, keyscan, debounce, mouse, hooks.slave),
                    )
                    .await;
                }
            } else {
                master::start(
                    m2s_tx,
                    s2m_rx,
                    ble,
                    usb,
                    keyscan,
                    debounce,
                    encoder,
                    storage,
                    mouse,
                    key_config,
                    hand,
                    hooks.master,
                )
                .await;
            }
        },
    )
    .await;
}
