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
        select {
            class: "select-sm select-bordered w-full",
            onchange: move |evt| {
                let Ok(idx) = evt.data().value().parse::<usize>() else {
                    return;
                };
                select_key(items[idx].clone());
            },
            for (i , item) in items.iter().enumerate() {
                option { value: i, selected: item == &selected_key, "{item}" }
            }
        }
    }
}

#[component]
pub fn LayerKeySelector(selected_key: LayerOp, select_key: Callback<LayerOp>) -> Element {
    rsx! {
        div { class: "grid grid-cols-3 items-center gap-2",
            select {
                class: "col-span-1 select select-sm select-bordered",
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
