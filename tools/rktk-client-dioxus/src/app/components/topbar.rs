use dioxus::prelude::*;

#[component]
pub fn Topbar() -> Element {
    rsx! {
        div { class: "flex bg-accent",
            h1 { class: "text-sm font-bold", "RKTK Client" }
            ThemeToggle {}
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
