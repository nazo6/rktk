use std::collections::HashMap;

use dioxus::prelude::*;

use fetcher::KeymapData;
use rktk_rrp::endpoints::{get_keyboard_info::KeyboardInfo, rktk_keymanager::keycode::KeyAction};

use crate::app::{components::selector::key_action::KeyActionSelector, state::CONN};

mod bar;
mod fetcher;
mod keyboard;

#[component]
pub fn Remap() -> Element {
    let mut res = use_resource(|| async { (fetcher::get_keymap().await, js_sys::Date::now()) });

    let keyboard = CONN
        .read()
        .as_ref()
        .context("Not connected")?
        .keyboard
        .clone();

    match &*res.value().read() {
        Some((Ok(keymap), time)) => {
            dioxus::logger::tracing::info!("{:?}", time);
            rsx! {
                div { class: "h-full",
                    // Using array as re-rendering using key only works for list
                    {[rsx! {
                        RemapInner {
                            keyboard,
                            keymap: keymap.to_owned(),
                            refetch: Callback::new(move |_| res.restart()),
                            key: "{time}",
                        }
                    }].iter()}
                }
            }
        }
        None => {
            rsx! {
                div {
                    h1 { "Loading" }
                    p { "Loading keymap" }
                }
            }
        }
        Some((Err(e), _)) => {
            dioxus::logger::tracing::error!("{:?}", e);
            rsx! {
                div {
                    h1 { "Error" }
                    p { "Failed to load keymap" }
                }
            }
        }
    }
}

#[component]
pub fn RemapInner(keyboard: KeyboardInfo, keymap: KeymapData, refetch: Callback<()>) -> Element {
    let mut modified_keymap = use_signal(|| keymap.clone());
    let mut keymap_changes = use_signal(HashMap::new);

    let selected: Signal<Option<(usize, usize)>> = use_signal(|| None);
    let layer = use_signal(|| 0);

    rsx! {
        div { class: "h-full flex flex-col items-center gap-2",
            bar::Bar {
                changes: keymap_changes.read().clone(),
                apply: Callback::new(move |_| {
                    spawn(async move {
                        if fetcher::set_keymap(&keymap_changes.read()).await.is_ok() {
                            refetch(());
                        }
                    });
                }),
                discard_all: Callback::new({
                    let keymap = keymap.clone();
                    move |_| {
                        {
                            keymap_changes.write().clear();
                            modified_keymap.set(keymap.clone());
                        }
                    }
                }),
            }
            keyboard::Keyboard {
                layer,
                keymap: modified_keymap.read().clone(),
                select_signal: selected,
                keymap_changes,
            }
            div {
                match *selected.read() {
                    Some((row, col)) => {
                        let orig_key = keymap[*layer.read()][row][col].clone();
                        rsx! {
                            if let Some(key_action) = modified_keymap.read()[*layer.read()][row][col].action {
                                div {
                                    code { {format!("Layer: {}, Row: {}, Col: {}", *layer.read(), row, col)} }
                                }
                                KeyActionSelector {
                                    key_action,
                                    discard: Callback::new(move |_| {
                                        let layer = *layer.read();
                                        keymap_changes.write().remove(&(layer as u8, row as u8, col as u8));
                                        (*modified_keymap.write())[layer][row][col].action = orig_key.action;
                                    }),
                                    select_key_action: Callback::new(move |ka: KeyAction| {
                                        let layer = *layer.read();
                                        if keymap[layer][row][col].action == Some(ka) {
                                            keymap_changes.write().remove(&(layer as u8, row as u8, col as u8));
                                        } else {
                                            keymap_changes.write().insert((layer as u8, row as u8, col as u8), ka);
                                        }
                                        (*modified_keymap.write())[layer][row][col].action = Some(ka);
                                    }),
                                }
                            }
                        }
                    }
                    None => {
                        rsx! {
                            p { "Select a key to remap" }
                        }
                    }
                }
            }
        }
    }
}
