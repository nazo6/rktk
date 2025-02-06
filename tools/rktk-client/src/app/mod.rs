use dioxus::prelude::*;
use js_sys::wasm_bindgen::{prelude::Closure, JsCast as _};

use crate::TAILWIND_CSS;

mod cache;
mod components;
mod disconnect;
mod page;
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");

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
    use_effect(|| {
        let window = web_sys::window().expect("Missing Window");
        let hid = window.navigator().hid();
        if !hid.is_falsy() {
            let cb = Closure::wrap(Box::new(move || {
                spawn_forever(async move {
                    let _ = disconnect::disconnect().await;
                });
            }) as Box<dyn FnMut()>);

            hid.set_ondisconnect(Some(cb.as_ref().unchecked_ref()));

            cb.forget();
        }
    });

    let window = web_sys::window().expect("Missing Window");
    let hid = window.navigator().hid();

    rsx! {
        if hid.is_falsy() {
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
