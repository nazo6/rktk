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
    config::{SPLIT_CHANNEL_SIZE, SPLIT_USB_TIMEOUT},
    constant::LAYER_COUNT,
    interface::{
        keyscan::Keyscan,
        mouse::Mouse,
        split::{MasterToSlave, SlaveToMaster, SplitDriver},
        usb::UsbDriver,
    },
    keycode::Layer,
};

mod master;
mod slave;
mod split_handler;

type S2mChannel = Channel<CriticalSectionRawMutex, SlaveToMaster, SPLIT_CHANNEL_SIZE>;
type S2mRx<'a> = Receiver<'a, CriticalSectionRawMutex, SlaveToMaster, SPLIT_CHANNEL_SIZE>;
type S2mTx<'a> = Sender<'a, CriticalSectionRawMutex, SlaveToMaster, SPLIT_CHANNEL_SIZE>;

type M2sChannel = Channel<CriticalSectionRawMutex, MasterToSlave, SPLIT_CHANNEL_SIZE>;
type M2sRx<'a> = Receiver<'a, CriticalSectionRawMutex, MasterToSlave, SPLIT_CHANNEL_SIZE>;
type M2sTx<'a> = Sender<'a, CriticalSectionRawMutex, MasterToSlave, SPLIT_CHANNEL_SIZE>;

pub async fn start<KS: Keyscan, M: Mouse, USB: UsbDriver, SP: SplitDriver>(
    keymap: [Layer; LAYER_COUNT],
    key_scanner: KS,
    mouse: Option<M>,
    split: SP,
    mut usb: USB,
) {
    let is_master = match select(usb.wait_ready(), Timer::after(SPLIT_USB_TIMEOUT)).await {
        Either::First(_) => true,
        Either::Second(_) => false,
    };

    let s2m_chan: S2mChannel = Channel::new();
    let s2m_tx = s2m_chan.sender();
    let s2m_rx = s2m_chan.receiver();

    let m2s_chan: M2sChannel = Channel::new();
    let m2s_tx = m2s_chan.sender();
    let m2s_rx = m2s_chan.receiver();

    crate::utils::display_state!(Master, is_master);

    if is_master {
        join(
            split_handler::start(split, s2m_tx, m2s_rx),
            master::start(m2s_tx, s2m_rx, keymap, key_scanner, mouse, usb),
        )
        .await;
    } else {
        join(
            split_handler::start(split, m2s_tx, s2m_rx),
            slave::start(s2m_tx, m2s_rx, key_scanner, mouse),
        )
        .await;
    }
}
