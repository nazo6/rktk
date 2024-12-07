use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct ConnectedState {
    client: Signal<rktk_rrp_client_webhid::Client>,
}
