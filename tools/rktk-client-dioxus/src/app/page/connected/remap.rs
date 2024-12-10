use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_query::prelude::*;

use futures::Stream;
use rktk_rrp::endpoints::{
    get_keyboard_info::KeyboardInfo, rktk_keymanager::keycode::KeyAction, KeyActionLoc,
};

use crate::app::{
    components::selector::key_action::KeyActionSelector,
    query::query::{get_keymap::KeymapData, use_app_get_query, QueryKey, QueryValue},
    state::CONN,
};

mod bar;
mod keyboard;

#[component]
pub fn Remap() -> Element {
    let res = use_app_get_query([QueryKey::GetKeymap]);

    let keyboard = CONN
        .read()
        .as_ref()
        .context("Not connected")?
        .keyboard
        .clone();

    let res = res.result();
    match res.value() {
        QueryResult::Ok(QueryValue::Keymap(keymap)) => {
            rsx! {
                div { class: "h-full",
                    RemapInner { keyboard, keymap: keymap.to_owned() }
                }
            }
        }
        QueryResult::Loading(_) => {
            rsx! {
                div {
                    h1 { "Loading" }
                    p { "Loading keymap" }
                }
            }
        }
        QueryResult::Err(e) => {
            dioxus::logger::tracing::error!("{:?}", e);
            rsx! {
                div {
                    h1 { "Error" }
                    p { "Failed to load keymap" }
                }
            }
        }
        _ => unreachable!(),
    }
}

#[component]
pub fn RemapInner(keyboard: KeyboardInfo, keymap: KeymapData) -> Element {
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
                        if let Some(c) = &*CONN.read() {
                            let changes: Vec<KeyActionLoc> = keymap_changes
                                .read()
                                .iter()
                                .map(|(k, v)| {
                                    KeyActionLoc {
                                        layer: k.0,
                                        row: k.1,
                                        col: k.2,
                                        key: v.clone(),
                                    }
                                })
                                .collect();
                            let changes = futures::stream::iter(changes);
                            let res = c.client.client.lock().await.set_keymaps(changes).await;
                            dioxus::logger::tracing::info!("{:?}", res);
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
            }
            div {
                match *selected.read() {
                    Some((row, col)) => {
                        let orig_key = keymap[*layer.read()][row][col].clone();
                        rsx! {
                            if let Some(key_action) = modified_keymap.read()[*layer.read()][row][col].action {
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
