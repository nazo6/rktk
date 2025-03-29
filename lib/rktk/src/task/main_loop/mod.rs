use crate::{
    config::{
        constant::{KEYBOARD, RKTK_CONFIG},
        keymap::Keymap,
    },
    drivers::interface::{
        ble::BleDriver, debounce::DebounceDriver, encoder::EncoderDriver, keyscan::KeyscanDriver,
        mouse::MouseDriver, rgb::RgbDriver, split::SplitDriverBuilder, storage::StorageDriver,
        system::SystemDriver, usb::UsbDriver,
    },
    hooks::{
        Hooks,
        interface::{CommonHooks, MasterHooks, RgbHooks, SlaveHooks},
    },
    interface::Hand,
    task::channels::split::{M2S_CHANNEL, S2M_CHANNEL},
    utils::sjoin,
};
use embassy_futures::select::{Either, select};
use embassy_time::{Duration, Timer};
use rktk_log::debug;

mod master;
mod rgb;
mod slave;
mod split_handler;

#[allow(clippy::too_many_arguments)]
pub async fn start<CH: CommonHooks, MH: MasterHooks, SH: SlaveHooks, BH: RgbHooks>(
    system: &impl SystemDriver,
    ble: Option<impl BleDriver>,
    usb: Option<impl UsbDriver>,
    mut keyscan: impl KeyscanDriver,
    debounce: &mut Option<impl DebounceDriver>,
    encoder: Option<impl EncoderDriver>,
    mut mouse: Option<impl MouseDriver>,
    mut storage: Option<impl StorageDriver>,
    split: Option<impl SplitDriverBuilder>,
    rgb: Option<impl RgbDriver>,
    key_config: &Keymap,
    mut hooks: Hooks<CH, MH, SH, BH>,
    hand: Hand,
) {
    crate::utils::display_state!(Hand, Some(hand));

    let split = if let Some(split) = split {
        split.build().await.ok()
    } else {
        None
    };

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
