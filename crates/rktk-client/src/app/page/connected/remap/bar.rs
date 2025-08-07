use std::collections::HashMap;

use dioxus::prelude::*;
use rktk_keymanager::keycode::KeyAction;

#[component]
pub fn Bar(
    changes: HashMap<(u8, u8, u8), KeyAction>,
    apply: Callback<()>,
    discard_all: Callback<()>,
) -> Element {
    rsx! {
        div { class: "bg-base-300 text-secondary-content flex w-full h-10 items-center px-2 gap-2",
            div { class: "ml-auto p-2" }
            if !changes.is_empty() {
                div { "Pending changes: {changes.len()}" }
            }
            button {
                disabled: changes.is_empty(),
                class: "btn btn-sm btn-primary",
                onclick: move |_| apply(()),
                "Apply"
            }
            button {
                disabled: changes.is_empty(),
                class: "btn btn-sm btn-secondary",
                onclick: move |_| discard_all(()),
                "Discard"
            }
        }
    }
}
