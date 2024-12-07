use dioxus::prelude::*;

use crate::app::{disconnect::disconnect, state::CONN};

#[component]
pub fn Topbar() -> Element {
    rsx! {
        div { class: "flex bg-accent text-accent-content items-center px-2",
            h1 { class: "text-2xl font-bold", "RKTK Client" }
            div { class: "ml-auto flex gap-2",
                if CONN.read().is_some() {
                    "Connected"
                    button {
                        class: "btn btn-outline btn-xs",
                        onclick: move |_| {
                            spawn(async move {
                                match disconnect().await {
                                    Ok(_) => {
                                        *CONN.write() = None;
                                    }
                                    Err(e) => {
                                        dioxus::logger::tracing::info!("{:?}", e);
                                    }
                                }
                            });
                        },
                        "Disconnect"
                    }
                } else {
                    "Disconnected"
                }
            }
            div { class: "ml-auto flex items-center", ThemeToggle {} }
        }
    }
}

#[component]
fn ThemeToggle() -> Element {
    rsx! {
        input {
            r#type: "checkbox",
            class: "toggle theme-controller",
            value: "dark",
        }
    }
}
