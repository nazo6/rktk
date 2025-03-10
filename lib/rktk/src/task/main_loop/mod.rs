use crate::{
    config::{
        constant::{KEYBOARD, RKTK_CONFIG},
        keymap::Keymap,
    },
    drivers::interface::{
        ble::BleDriver,
        debounce::DebounceDriver,
        encoder::EncoderDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        rgb::RgbDriver,
        split::SplitDriver,
        storage::StorageDriver,
        system::SystemDriver,
        usb::UsbDriver,
    },
    hooks::{
        interface::{CommonHooks, MasterHooks, RgbHooks, SlaveHooks},
        Hooks,
    },
    task::channels::split::{M2S_CHANNEL, S2M_CHANNEL},
    utils::sjoin,
};
use embassy_futures::select::{select, Either};
use embassy_time::{Duration, Timer};
use rktk_keymanager::state::hooks::Hooks as KeymanagerHooks;
use rktk_log::debug;

mod master;
mod rgb;
mod slave;
mod split_handler;

#[allow(clippy::too_many_arguments)]
pub async fn start<
    Sys: SystemDriver,
    KS: KeyscanDriver,
    DB: DebounceDriver,
    EN: EncoderDriver,
    M: MouseDriver,
    SP: SplitDriver,
    RGB: RgbDriver,
    Ble: BleDriver,
    Usb: UsbDriver,
    S: StorageDriver,
    CH: CommonHooks,
    MH: MasterHooks,
    SH: SlaveHooks,
    BH: RgbHooks,
    KH: KeymanagerHooks,
>(
    system: &Sys,
    ble: Option<Ble>,
    usb: Option<Usb>,
    mut keyscan: KS,
    debounce: Option<DB>,
    encoder: Option<EN>,
    mut mouse: Option<M>,
    mut storage: Option<S>,
    mut split: Option<SP>,
    rgb: Option<RGB>,
    key_config: Keymap,
    mut hooks: Hooks<CH, MH, SH, BH, KH>,
) {
    let hand = keyscan.current_hand().await;
    crate::utils::display_state!(Hand, Some(hand));

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

    let rgb_m2s_tx = if is_master {
        Some(M2S_CHANNEL.sender())
    } else {
        None
    };

    sjoin::join!(
        async move {
            if is_master {
                debug!("master start");
                master::start(
                    m2s_tx,
                    s2m_rx,
                    system,
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
                    hooks.key_manager,
                )
                .await;
            } else {
                debug!("slave start");
                slave::start(s2m_tx, m2s_rx, keyscan, debounce, mouse, hooks.slave).await;
            }
        },
        async move {
            if let Some(split) = split {
                debug!("split init");
                if is_master {
                    split_handler::start(split, s2m_tx, m2s_rx, is_master).await;
                } else {
                    split_handler::start(split, m2s_tx, s2m_rx, is_master).await;
                }
            } else {
                debug!("no split");
            }
        },
        async move {
            if let Some(rgb) = rgb {
                debug!("rgb init");
                match hand {
                    Hand::Right => {
                        rgb::start::<{ KEYBOARD.right_rgb_count }>(rgb, hooks.rgb, rgb_m2s_tx).await
                    }
                    Hand::Left => {
                        rgb::start::<{ KEYBOARD.left_rgb_count }>(rgb, hooks.rgb, rgb_m2s_tx).await
                    }
                }
            } else {
                debug!("no rgb");
            }
        }
    );
}
