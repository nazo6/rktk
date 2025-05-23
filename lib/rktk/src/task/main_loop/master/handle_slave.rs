use rktk_keymanager::interface::state::input_event::KeyChangeEvent;
use rktk_log::debug;

use crate::{
    config::schema::DynamicConfig,
    drivers::interface::split::SlaveToMaster,
    config::Hand,
    task::{
        channels::{
            report::{KEYBOARD_EVENT_REPORT_CHANNEL, update_mouse},
            split::S2mRx,
        },
        main_loop::master::utils::get_split_right_shift,
    },
};

use super::utils::resolve_entire_key_pos;

pub async fn start(config: &'static DynamicConfig, hand: Hand, s2m_rx: S2mRx<'_>) {
    debug!("split recv start");

    let slave_hand = hand.other();
    let shift = get_split_right_shift(config);
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
                    resolve_entire_key_pos(&mut ev, slave_hand, shift);
                    KEYBOARD_EVENT_REPORT_CHANNEL.send(ev).await;
                }
                SlaveToMaster::Released(row, col) => {
                    let mut ev = KeyChangeEvent {
                        col,
                        row,
                        pressed: false,
                    };
                    resolve_entire_key_pos(&mut ev, slave_hand, shift);
                    KEYBOARD_EVENT_REPORT_CHANNEL.send(ev).await;
                }
                SlaveToMaster::Mouse { x, y } => {
                    update_mouse(x, y);
                }
                SlaveToMaster::Message(_) => {}
            }
        }
    }
}
