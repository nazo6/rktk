use std::collections::HashMap;

use dioxus::prelude::*;

use fetcher::KeymapData;
use rktk_rrp::endpoints::{get_keyboard_info::KeyboardInfo, rktk_keymanager::keycode::KeyAction};

use crate::app::{
    cache::{invalidate_cache, use_cache, with_cache},
    components::{
        notification::{Notification, NotificationLevel, push_notification},
        selector::key_action::KeyActionSelector,
    },
    state::CONN,
};

mod bar;
mod fetcher;
mod keyboard;

#[component]
pub fn Remap() -> Element {
    let cache = use_cache();
    let mut res = use_resource({
        let cache = cache.clone();
        move || {
            with_cache(cache.clone(), "get_keymap", async {
                match fetcher::get_keymap().await {
                    Ok(data) => Ok((data, jiff::Zoned::now())),
                    Err(e) => Err(e),
                }
            })
        }
    });

    let keyboard = CONN
        .read()
        .as_ref()
        .context("Not connected")?
        .keyboard
        .clone();

    match &*res.value().read() {
        Some(Ok((keymap, time))) => {
            let elements = [rsx! {
                RemapInner {
                    keyboard,
                    keymap: keymap.to_owned(),
                    refetch: Callback::new(move |_| {
                        invalidate_cache(cache.clone(), "get_keymap");
                        res.restart()
                    }),
                    key: "{time}",
                }
            }];

            rsx! {
                div { class: "h-full",
                    // Using array as re-rendering using key only works for list
                    {elements.iter()}
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
        Some(Err(e)) => {
            rsx! {
                div {
                    h1 { "Error" }
                    p { "Failed to load keymap" }
                    p { "{e:?}" }

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
                        match fetcher::set_keymap(&keymap_changes.read()).await {
                            Ok(_) => {
                                push_notification(Notification {
                                    message: "Keymap updated".to_string(),
                                    level: NotificationLevel::Info,
                                    ..Default::default()
                                });
                                refetch(())
                            }
                            Err(e) => {
                                push_notification(Notification {
                                    message: format!("Cannot connect to device: {e:?}"),
                                    level: NotificationLevel::Error,
                                    ..Default::default()
                                });
                            }
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
