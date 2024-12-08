use dioxus::prelude::*;

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
    let mut tab = use_signal(|| Tabs::Remap);

    rsx! {
        div { class: "flex flex-col h-full",
            div { role: "tablist", class: "tabs tabs-lifted pt-2",
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
            div { class: "flex-grow",
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
