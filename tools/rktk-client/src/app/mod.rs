use std::sync::LazyLock;

use dioxus::prelude::*;

use crate::{
    TAILWIND_CSS,
    backend::{Backend, RrpHidBackend as _},
};

mod cache;
mod components;
mod disconnect;
mod page;
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");

static BACKEND: LazyLock<(Backend, async_channel::Receiver<()>)> = LazyLock::new(Backend::new);

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "RKTK Client" }
        components::notification::NotificationProvider {}
        div { class: "h-full bg-base flex flex-col",
            components::topbar::Topbar {}
            Home {}
        }
    }
}

#[component]
fn Home() -> Element {
    use_effect(move || {
        let mut rx = BACKEND.1.clone();
        spawn_forever(async move {
            while let Ok(_) = rx.recv().await {
                let _ = disconnect::disconnect().await;
            }
        });
    });

    rsx! {
        if Backend::available() {
            if state::CONN.read().is_some() {
                page::connected::Connected {}
            } else {
                page::connect::Connect {}
            }
        } else {
            h1 { "WebHID not supported" }
        }
    }
}
