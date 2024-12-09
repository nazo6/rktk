use dioxus::prelude::*;

use crate::TAILWIND_CSS;

mod components;
mod disconnect;
mod page;
mod query;
mod state;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[component]
pub fn App() -> Element {
    dioxus_query::prelude::use_init_query_client::<
        query::query::QueryValue,
        query::query::QueryError,
        query::query::QueryKey,
    >();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "RKTK Client" }
        div { class: "h-full bg-base flex flex-col",
            components::topbar::Topbar {}
            Home {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        if state::CONN.read().is_some() {
            page::connected::Connected {}
        } else {
            page::connect::Connect {}
        }
    }
}
