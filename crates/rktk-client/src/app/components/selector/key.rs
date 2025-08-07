use std::fmt::Display;

use dioxus::prelude::*;
use rktk_keymanager::keycode::layer::LayerOp;

#[component]
pub fn KeySelector<I: Display + PartialEq + Clone + 'static>(
    items: Vec<I>,
    selected_key: I,
    select_key: Callback<I>,
) -> Element {
    rsx! {
        div { class: "flex flex-row flex-wrap items-center gap-2 max-h-72 overflow-y-auto justify-center text",
            for item in items.into_iter() {
                div {
                    class: "border-2 p-1 text-sm font-bold cursor-pointer w-18 h-10 overflow-clip hover:bg-gray-500/20 ",
                    class: if item == selected_key { "border-accent" } else { "border-base-content" },
                    onclick: move |_| select_key(item.clone()),
                    "{item}"
                }
            }
        }
    }
}

#[component]
pub fn LayerKeySelector(selected_key: LayerOp, select_key: Callback<LayerOp>) -> Element {
    rsx! {
        div { class: "grid grid-cols-3 items-center gap-2",
            select {
                class: "col-span-1 select select-sm",
                onchange: move |evt| {
                    let selected_key = match evt.data().value().as_str() {
                        "mo" => LayerOp::Momentary(0),
                        "to" => LayerOp::Toggle(0),
                        _ => return,
                    };
                    select_key(selected_key);
                },
                option {
                    value: "mo",
                    selected: matches!(selected_key, LayerOp::Momentary(_)),
                    "Momentary"
                }
                option {
                    value: "to",
                    selected: matches!(selected_key, LayerOp::Toggle(_)),
                    "Toggle"
                }
            }
            input {
                class: "col-span-2 input input-sm input-bordered w-full",
                r#type: "number",
                value: match selected_key {
                    LayerOp::Momentary(n) | LayerOp::Toggle(n) => n.to_string(),
                },
                oninput: move |evt| {
                    let Ok(n) = evt.data().value().parse::<u8>() else {
                        return;
                    };
                    select_key(
                        match selected_key {
                            LayerOp::Momentary(_) => LayerOp::Momentary(n),
                            LayerOp::Toggle(_) => LayerOp::Toggle(n),
                        },
                    );
                },
            }
        }
    }
}

#[component]
pub fn CustomKeySelector(selected_key: u8, select_key: Callback<u8>) -> Element {
    rsx! {
        input {
            class: "col-span-2 input input-sm input-bordered w-full",
            r#type: "number",
            value: selected_key,
            oninput: move |evt| {
                let Ok(n) = evt.data().value().parse::<u8>() else {
                    return;
                };
                select_key(n);
            },
        }
    }
}
