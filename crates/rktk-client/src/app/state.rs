use dioxus::prelude::*;
use futures::lock::Mutex;
use rktk_rrp::endpoints::get_keyboard_info::KeyboardInfo;

use crate::backend::{Backend, RrpHidBackend};

pub struct ConnectedState {
    pub device: Mutex<<Backend as RrpHidBackend>::HidDevice>,
    pub keyboard: KeyboardInfo,
}

pub static CONN: GlobalSignal<Option<ConnectedState>> = GlobalSignal::new(|| None);
