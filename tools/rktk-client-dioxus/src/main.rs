mod app;

use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::INFO).unwrap();
    dioxus::launch(app::App);
}
