use dioxus::prelude::*;
use rktk_rrp::endpoints::KeyActionLoc;

#[component]
pub fn Bar(changes: Vec<KeyActionLoc>) -> Element {
    rsx! {
        div { class: "text-secondary-content bg-secondary flex w-full h-10 items-center px-2 gap-2",
            if changes.len() > 0 {
                div { class: "ml-auto gap-2 p-2", "Changes: {changes.len()}" }
                button { class: "btn btn-sm", "Apply" }
                button { class: "btn btn-sm", "Discard" }
            }
        }
    }
}
