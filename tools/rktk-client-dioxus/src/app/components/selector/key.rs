use dioxus::prelude::*;

#[component]
pub fn KeySelector<I: Into<&'static str> + PartialEq + 'static>(items: Vec<I>) -> Element {
    rsx! {
        select {
            for item in items {
                option { {item.into()} }
            }
        }
    }
}
