use dioxus::prelude::*;
use rktk_rrp_client_webhid::Client;
use web_sys::HidDevice;

pub struct ConnectedState {
    pub client: Client,
    pub device: HidDevice,
}

pub static CONN: GlobalSignal<Option<ConnectedState>> = GlobalSignal::new(|| None);
