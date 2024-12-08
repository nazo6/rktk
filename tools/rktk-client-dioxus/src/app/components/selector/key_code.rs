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
        div { class: "flex flex-col",
            div {
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::None),
                    onclick: move |_| select_key_code(KeyCode::None),
                    "None"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::Key(_)),
                    onclick: move |_| select_key_code(KeyCode::Key(Key::A)),
                    "Key"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::Mouse(_)),
                    onclick: move |_| select_key_code(KeyCode::Mouse(Mouse::Left)),
                    "Mouse"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::Modifier(_)),
                    onclick: move |_| select_key_code(KeyCode::Modifier(Modifier::LShft)),
                    "Modifier"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::Layer(_)),
                    onclick: move |_| select_key_code(KeyCode::Layer(LayerOp::Momentary(0))),
                    "Layer"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::Special(_)),
                    onclick: move |_| select_key_code(KeyCode::Special(Special::MoScrl)),
                    "Special"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_code, KeyCode::Media(_)),
                    onclick: move |_| select_key_code(KeyCode::Media(Media::Play)),
                    "Media"
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
                    KeyCode::Mouse(mouse) => rsx! {},
                    KeyCode::Modifier(modifier) => rsx! {},
                    KeyCode::Layer(layer_op) => rsx! {},
                    KeyCode::Special(special) => rsx! {},
                    KeyCode::Media(media) => rsx! {},
                }
            }
        }
    }
}
