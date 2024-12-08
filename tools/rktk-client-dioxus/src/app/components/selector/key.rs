use std::fmt::Display;

use dioxus::prelude::*;

#[component]
pub fn KeySelector<I: Display + PartialEq + Clone + 'static>(
    items: Vec<I>,
    selected_key: I,
    select_key: Callback<I>,
) -> Element {
    rsx! {
        select {
            for item in items {
                option { "{item}" }
            }
        }
    }
}
