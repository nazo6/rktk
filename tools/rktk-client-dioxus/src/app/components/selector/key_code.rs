use dioxus::prelude::*;
use rktk_keymanager::keycode::{
    key::Key, layer::LayerOp, media::Media, modifier::Modifier, mouse::Mouse, special::Special,
    KeyCode,
};
use strum::IntoEnumIterator as _;

use super::key::KeySelector;

#[component]
pub fn KeyCodeSelector(key_code: KeyCode, select_key_code: Callback<KeyCode>) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2", class: "join",
            form {
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::None),
                    onclick: move |_| select_key_code(KeyCode::None),
                    aria_label: "None",
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::Key(_)),
                    onclick: move |_| select_key_code(KeyCode::Key(Key::A)),
                    aria_label: "Key",
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::Mouse(_)),
                    onclick: move |_| select_key_code(KeyCode::Mouse(Mouse::MLeft)),
                    aria_label: "Mouse",
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::Modifier(_)),
                    onclick: move |_| select_key_code(KeyCode::Modifier(Modifier::LShft)),
                    aria_label: "Modifier",
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::Layer(_)),
                    onclick: move |_| select_key_code(KeyCode::Layer(LayerOp::Momentary(0))),
                    aria_label: "Layer",
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::Special(_)),
                    onclick: move |_| select_key_code(KeyCode::Special(Special::MoScrl)),
                    aria_label: "Special",
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn btn-sm",
                    checked: matches!(key_code, KeyCode::Media(_)),
                    onclick: move |_| select_key_code(KeyCode::Media(Media::Play)),
                    aria_label: "Media",
                }
            }
            div {
                match key_code {
                    KeyCode::None => rsx! {},
                    KeyCode::Key(key) => rsx! {
                        KeySelector {
                            items: Key::iter().collect(),
                            selected_key: key,
                            select_key: Callback::new(move |key| select_key_code(KeyCode::Key(key))),
                        }
                    },
                    KeyCode::Mouse(mouse) => rsx! {
                        KeySelector {
                            items: Mouse::iter().collect(),
                            selected_key: mouse,
                            select_key: Callback::new(move |mouse| select_key_code(KeyCode::Mouse(mouse))),
                        }
                    },
                    KeyCode::Modifier(modifier) => rsx! {
                        KeySelector {
                            items: Modifier::iter().collect(),
                            selected_key: modifier,
                            select_key: Callback::new(move |modifier| select_key_code(KeyCode::Modifier(modifier))),
                        }
                    },
                    KeyCode::Layer(layer_op) => rsx! {},
                    KeyCode::Special(special) => rsx! {
                        KeySelector {
                            items: Special::iter().collect(),
                            selected_key: special,
                            select_key: Callback::new(move |special| select_key_code(KeyCode::Special(special))),
                        }
                    },
                    KeyCode::Media(media) => rsx! {
                        KeySelector {
                            items: Media::iter().collect(),
                            selected_key: media,
                            select_key: Callback::new(move |media| select_key_code(KeyCode::Media(media))),
                        }
                    },
                }
            }
        }
    }
}
