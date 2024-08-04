use embassy_futures::{
    join::join,
    select::{select, Either},
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::Timer;

use crate::{
    config::static_config::CONFIG,
    interface::{
        backlight::BacklightDriver,
        ble::BleDriver,
        keyscan::{Hand, KeyscanDriver},
        mouse::MouseDriver,
        split::{MasterToSlave, SlaveToMaster, SplitDriver},
        usb::UsbDriver,
    },
    keycode::Layer,
};

mod master;
mod slave;
mod split_handler;

type S2mChannel = Channel<CriticalSectionRawMutex, SlaveToMaster, { CONFIG.split_channel_size }>;
type S2mRx<'a> =
    Receiver<'a, CriticalSectionRawMutex, SlaveToMaster, { CONFIG.split_channel_size }>;
type S2mTx<'a> = Sender<'a, CriticalSectionRawMutex, SlaveToMaster, { CONFIG.split_channel_size }>;

type M2sChannel = Channel<CriticalSectionRawMutex, MasterToSlave, { CONFIG.split_channel_size }>;
type M2sRx<'a> =
    Receiver<'a, CriticalSectionRawMutex, MasterToSlave, { CONFIG.split_channel_size }>;
type M2sTx<'a> = Sender<'a, CriticalSectionRawMutex, MasterToSlave, { CONFIG.split_channel_size }>;

pub async fn start<
    KS: KeyscanDriver,
    M: MouseDriver,
    USB: UsbDriver,
    SP: SplitDriver,
    BL: BacklightDriver,
    BT: BleDriver,
>(
    keymap: [Layer; CONFIG.layer_count],
    mut key_scanner: KS,
    mouse: Option<M>,
    mut split: SP,
    mut usb: USB,
    backlight: Option<BL>,
    mut ble: Option<BT>,
) {
    let hand = key_scanner.current_hand().await;
    crate::utils::display_state!(Hand, Some(hand));

    join(
        async move {
            if let Some(backlight) = backlight {
                match hand {
                    Hand::Right => {
                        super::backlight::start::<{ CONFIG.right_led_count }>(backlight).await
                    }
                    Hand::Left => {
                        super::backlight::start::<{ CONFIG.left_led_count }>(backlight).await
                    }
                }
            }
        },
        async move {
            let _ = split.init().await;

            let is_master = if let Some(ble) = &mut ble {
                let _ = ble.wait_ready().await;
                true
            } else {
                match select(
                    usb.wait_ready(),
                    Timer::after_millis(CONFIG.split_usb_timeout),
                )
                .await
                {
                    Either::First(_) => true,
                    Either::Second(_) => false,
                }
            };

            crate::utils::display_state!(Master, Some(is_master));

            let s2m_chan: S2mChannel = Channel::new();
            let s2m_tx = s2m_chan.sender();
            let s2m_rx = s2m_chan.receiver();

            let m2s_chan: M2sChannel = Channel::new();
            let m2s_tx = m2s_chan.sender();
            let m2s_rx = m2s_chan.receiver();

            if is_master {
                join(
                    split_handler::start(split, s2m_tx, m2s_rx, is_master),
                    master::start(m2s_tx, s2m_rx, keymap, key_scanner, mouse, usb, hand),
                )
                .await;
            } else {
                join(
                    split_handler::start(split, m2s_tx, s2m_rx, is_master),
                    slave::start(s2m_tx, m2s_rx, key_scanner, mouse),
                )
                .await;
            }
        },
    )
    .await;
}
