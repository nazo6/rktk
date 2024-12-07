use dioxus::prelude::*;

use crate::TAILWIND_CSS;

mod components;
mod page;
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "RKTK Client a" }
        Home {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "h-full bg-base flex flex-col", components::topbar::Topbar {} }
    }
}
