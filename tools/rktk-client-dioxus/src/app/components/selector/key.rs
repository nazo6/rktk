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
                option { value: i, "{item}" }
            }
        }
    }
}

#[component]
pub fn LayerSelector(selected_key: LayerOp, select_key: Callback<LayerOp>) -> Element {
    rsx! {
        div {
            select {
                option { "Momentary" }
                option { "Toggle" }
            }
        }
    }
}
