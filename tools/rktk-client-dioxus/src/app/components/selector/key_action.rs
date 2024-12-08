use dioxus::prelude::*;
use rktk_keymanager::keycode::{key::Key, KeyAction, KeyCode};

#[component]
pub fn KeyActionSelector(key_action: KeyAction, select_key_action: Callback<KeyAction>) -> Element {
    rsx! {
        div { class: "flex flex-col",
            div {
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    onclick: move |_| select_key_action(KeyAction::Normal(KeyCode::Key(Key::A))),
                    checked: matches!(key_action, KeyAction::Normal(_)),
                    "Normal"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_action, KeyAction::Normal2(_, _)),
                    onclick: move |_| {
                        select_key_action(KeyAction::Normal2(KeyCode::Key(Key::A), KeyCode::Key(Key::A)))
                    },
                    "Normal2"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_action, KeyAction::OneShot(_)),
                    onclick: move |_| select_key_action(KeyAction::OneShot(KeyCode::Key(Key::A))),
                    "Oneshot"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_action, KeyAction::TapHold(_, _)),
                    onclick: move |_| {
                        select_key_action(KeyAction::TapHold(KeyCode::Key(Key::A), KeyCode::Key(Key::A)))
                    },
                    "Tap-Hold"
                }
                input {
                    r#type: "radio",
                    name: "options",
                    class: "join-item btn",
                    checked: matches!(key_action, KeyAction::TapDance(_)),
                    onclick: move |_| { select_key_action(KeyAction::TapDance(0)) },
                    "Tap-Dance"
                }
            }
            div {
                match key_action {
                    KeyAction::Inherit => rsx! {},
                    KeyAction::Normal(key_code) => rsx! {
                        div {}
                    },
                    KeyAction::Normal2(key_code, key_code1) => rsx! {
                        div {}
                    },
                    KeyAction::TapHold(key_code, key_code1) => rsx! {
                        div {}
                    },
                    KeyAction::OneShot(key_code) => rsx! {
                        div {}
                    },
                    KeyAction::TapDance(_) => rsx! {
                        div {}
                    },
                }
            }

        }
    }
}
