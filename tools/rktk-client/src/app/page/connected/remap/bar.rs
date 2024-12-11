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
        div { class: "text-secondary-content bg-secondary flex w-full h-10 items-center px-2 gap-2",
            if !changes.is_empty() {
                div { class: "ml-auto gap-2 p-2", "Changes: {changes.len()}" }
                button { class: "btn btn-sm", onclick: move |_| apply(()), "Apply" }
                button { class: "btn btn-sm", onclick: move |_| discard_all(()), "Discard" }
            }
        }
    }
}
