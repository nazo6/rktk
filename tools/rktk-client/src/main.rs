mod app;
mod backend;

use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::INFO).unwrap();

    tracing_log::LogTracer::builder()
        .with_max_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    dioxus::launch(app::App);
}
