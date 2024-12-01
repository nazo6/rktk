use rktk_keymanager::state::KeyChangeEvent;

use crate::{
    drivers::interface::{keyscan::Hand, split::SlaveToMaster},
    task::main_loop::S2mRx,
};

use super::{
    utils::resolve_entire_key_pos, KEYBOARD_EVENT_REPORT_CHANNEL, MOUSE_EVENT_REPORT_CHANNEL,
};

pub async fn start(hand: Hand, s2m_rx: S2mRx<'_>) {
    loop {
        s2m_rx.ready_to_receive().await;
        while let Ok(cmd_from_slave) = s2m_rx.try_receive() {
            match cmd_from_slave {
                SlaveToMaster::Pressed(row, col) => {
                    let mut ev = KeyChangeEvent {
                        col,
                        row,
                        pressed: true,
                    };
                    resolve_entire_key_pos(&mut ev, hand);
                    KEYBOARD_EVENT_REPORT_CHANNEL.send(ev).await;
                }
                SlaveToMaster::Released(row, col) => {
                    let mut ev = KeyChangeEvent {
                        col,
                        row,
                        pressed: false,
                    };
                    resolve_entire_key_pos(&mut ev, hand);
                    KEYBOARD_EVENT_REPORT_CHANNEL.send(ev).await;
                }
                SlaveToMaster::Mouse { x, y } => {
                    MOUSE_EVENT_REPORT_CHANNEL.send((x, y)).await;
                }
                SlaveToMaster::Message(_) => {}
            }
        }
    }
}
