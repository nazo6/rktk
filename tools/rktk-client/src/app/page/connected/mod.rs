use dioxus::prelude::*;

use crate::app::cache::use_cache_context_provider;

mod config;
mod log;
mod remap;

#[derive(PartialEq, Eq)]
enum Tabs {
    Remap,
    Config,
    Log,
}

#[component]
pub fn Connected() -> Element {
    use_cache_context_provider();

    let mut tab = use_signal(|| Tabs::Remap);

    rsx! {
        div { class: "flex flex-col h-full",
            div {
                role: "tablist",
                class: "tabs tabs-boxed ml-auto mr-auto my-2",
                a {
                    role: "tab",
                    class: "tab",
                    class: if *tab.read() == Tabs::Remap { "tab-active" },
                    onclick: move |_| tab.set(Tabs::Remap),
                    "Remap"
                }
                a {
                    role: "tab",
                    class: "tab",
                    class: if *tab.read() == Tabs::Config { "tab-active" },
                    onclick: move |_| tab.set(Tabs::Config),
                    "Config"
                }
                a {
                    role: "tab",
                    class: "tab",
                    class: if *tab.read() == Tabs::Log { "tab-active" },
                    onclick: move |_| tab.set(Tabs::Log),
                    "Log"
                }
            }
            div { class: "grow",
                match *tab.read() {
                    Tabs::Remap => rsx! {
                        remap::Remap {}
                    },
                    Tabs::Config => rsx! {
                        config::Config {}
                    },
                    Tabs::Log => rsx! {
                        log::Log {}
                    },
                }
            }
        }
    }
}
