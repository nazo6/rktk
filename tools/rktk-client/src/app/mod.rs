use std::sync::{LazyLock, Mutex};

use dioxus::prelude::*;

use crate::{
    backend::{Backend, RrpHidBackend as _},
    TAILWIND_CSS,
};

mod cache;
mod components;
mod disconnect;
mod page;
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");

static BACKEND: LazyLock<Mutex<Backend>> = LazyLock::new(|| Mutex::new(Backend::new()));

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
    let hid_available = {
        #[cfg(feature = "web")]
        {
            let window = web_sys::window().expect("Missing Window");
            let hid = window.navigator().hid();
            !hid.is_falsy()
        }
        #[cfg(feature = "native")]
        true
    };

    use_effect(move || {
        if hid_available {
            BACKEND.lock().unwrap().set_ondisconnect(Some(move || {
                spawn_forever(async move {
                    let _ = disconnect::disconnect().await;
                });
            }));
        }
    });

    rsx! {
        if hid_available {
            h1 { "WebHID not supported" }
        } else {
            if state::CONN.read().is_some() {
                page::connected::Connected {}
            } else {
                page::connect::Connect {}
            }
        }
    }
}
