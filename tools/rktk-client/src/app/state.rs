use dioxus::prelude::*;
use rktk_rrp::endpoints::get_keyboard_info::KeyboardInfo;
use rktk_rrp_client_webhid::Client;
use web_sys::HidDevice;

pub struct ConnectedState {
    pub client: Client,
    pub device: HidDevice,
    pub keyboard: KeyboardInfo,
}

pub static CONN: GlobalSignal<Option<ConnectedState>> = GlobalSignal::new(|| None);
