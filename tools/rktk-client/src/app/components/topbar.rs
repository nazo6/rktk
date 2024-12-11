use dioxus::prelude::*;

use crate::app::{
    components::notification::{push_notification, Notification, NotificationLevel},
    disconnect::disconnect,
    state::CONN,
};

#[component]
pub fn Topbar() -> Element {
    rsx! {
        div { class: "flex bg-primary text-primary-content items-center h-12 px-2",
            h1 { class: "text-2xl font-bold", "RKTK Client" }
            div { class: "ml-auto flex gap-2 items-center",
                if let Some(state) = &*CONN.read() {
                    "Connected: "
                    span { class: "font-bold text-xl", {state.keyboard.name.clone()} }
                    button {
                        class: "btn btn-outline btn-xs",
                        onclick: move |_| {
                            spawn(async move {
                                match disconnect().await {
                                    Ok(_) => {
                                        *CONN.write() = None;
                                    }
                                    Err(e) => {
                                        push_notification(Notification {
                                            message: format!("Cannot disconnect from device: {:?}", e),
                                            level: NotificationLevel::Error,
                                            ..Default::default()
                                        });
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
        div { class: "join bg-base-100 p-0.5",
            input {
                r#type: "radio",
                class: "join-item theme-controller btn btn-sm btn-secondary",
                checked: true,
                value: "default",
                name: "theme",
                aria_label: "Default",
            }
            input {
                r#type: "radio",
                class: "join-item theme-controller btn btn-sm btn-secondary",
                value: "cupcake",
                name: "theme",
                aria_label: "Light",
            }
            input {
                r#type: "radio",
                class: "join-item theme-controller btn btn-sm btn-secondary",
                value: "dark",
                name: "theme",
                aria_label: "Dark",
            }
        }
    }
}
