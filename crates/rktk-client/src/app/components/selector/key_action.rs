use dioxus::prelude::*;
use rktk_keymanager::keycode::{KeyAction, KeyCode, key::Key};

use crate::app::components::selector::key_code::KeyCodeSelector;

#[component]
pub fn KeyActionSelector(
    key_action: KeyAction,
    select_key_action: Callback<KeyAction>,
    discard: Callback<()>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 p-2 rounded-md border-2 items-center",
            div { class: "flex gap-2",
                form { class: "join",
                    input {
                        r#type: "radio",
                        name: "options",
                        class: "join-item btn btn-sm",
                        checked: matches!(key_action, KeyAction::Inherit),
                        onclick: move |_| select_key_action(KeyAction::Inherit),
                        aria_label: "Inherit",
                    }
                    input {
                        r#type: "radio",
                        name: "options",
                        class: "join-item btn btn-sm",
                        checked: matches!(key_action, KeyAction::Normal(_)),
                        onclick: move |_| select_key_action(KeyAction::Normal(KeyCode::Key(Key::A))),
                        aria_label: "Normal",
                    }
                    input {
                        r#type: "radio",
                        name: "options",
                        class: "join-item btn btn-sm",
                        checked: matches!(key_action, KeyAction::Normal2(_, _)),
                        onclick: move |_| {
                            select_key_action(KeyAction::Normal2(KeyCode::Key(Key::A), KeyCode::Key(Key::A)))
                        },
                        aria_label: "Normal2",
                    }
                    input {
                        r#type: "radio",
                        name: "options",
                        class: "join-item btn btn-sm",
                        checked: matches!(key_action, KeyAction::OneShot(_)),
                        onclick: move |_| select_key_action(KeyAction::OneShot(KeyCode::Key(Key::A))),
                        aria_label: "Oneshot",
                    }
                    input {
                        r#type: "radio",
                        name: "options",
                        class: "join-item btn btn-sm",
                        checked: matches!(key_action, KeyAction::TapHold(_, _)),
                        onclick: move |_| {
                            select_key_action(KeyAction::TapHold(KeyCode::Key(Key::A), KeyCode::Key(Key::A)))
                        },
                        aria_label: "Tap-Hold",
                    }
                    input {
                        r#type: "radio",
                        name: "options",
                        class: "join-item btn btn-sm",
                        checked: matches!(key_action, KeyAction::TapDance(_)),
                        onclick: move |_| { select_key_action(KeyAction::TapDance(0)) },
                        aria_label: "Tap-Dance",
                    }
                }
                button {
                    class: "btn btn-sm btn-secondary",
                    onclick: move |_| discard(()),
                    "Discard"
                }
            }
            div { class: "border-2 rounded-md p-2 w-full",
                match key_action {
                    KeyAction::Inherit => rsx! { "Inherit" },
                    KeyAction::Normal(key_code) => rsx! {
                        div {
                            KeyCodeSelector {
                                key_code,
                                select_key_code: Callback::new(move |kc| {
                                    select_key_action(KeyAction::Normal(kc));
                                }),
                            }
                        }
                    },
                    KeyAction::Normal2(key_code, key_code1) => rsx! {
                        div { class: "flex-col",
                            p { class: "text-xs", "First key" }
                            KeyCodeSelector {
                                key_code,
                                select_key_code: Callback::new(move |kc| {
                                    select_key_action(KeyAction::Normal2(kc, key_code1));
                                }),
                            }
                            p { class: "pt-4 text-xs", "Second key" }
                            KeyCodeSelector {
                                key_code: key_code1,
                                select_key_code: Callback::new(move |kc| {
                                    select_key_action(KeyAction::Normal2(key_code, kc));
                                }),
                            }
                        }
                    },
                    KeyAction::TapHold(key_code, key_code1) => rsx! {
                        div { class: "flex-col",
                            p { class: "text-center", "Tap key" }
                            KeyCodeSelector {
                                key_code,
                                select_key_code: Callback::new(move |kc| {
                                    select_key_action(KeyAction::TapHold(kc, key_code1));
                                }),
                            }
                            p { class: "pt-4 text-center", "Hold key" }
                            KeyCodeSelector {
                                key_code: key_code1,
                                select_key_code: Callback::new(move |kc| {
                                    select_key_action(KeyAction::TapHold(key_code, kc));
                                }),
                            }
                        }
                    },
                    KeyAction::OneShot(key_code) => rsx! {
                        div {
                            KeyCodeSelector {
                                key_code,
                                select_key_code: Callback::new(move |kc| {
                                    select_key_action(KeyAction::OneShot(kc));
                                }),
                            }
                        }
                    },
                    KeyAction::TapDance(id) => rsx! {
                        div { class: "flex gap-2 items-center",
                            "Tap-dance ID"
                            input {
                                r#type: "number",
                                class: "input input-bordered input-sm grow",
                                value: id,
                                onchange: move |evt| {
                                    let Ok(i) = evt.data().value().parse::<u8>() else {
                                        return;
                                    };
                                    select_key_action(KeyAction::TapDance(i))
                                },
                            }
                        }
                    },
                }
            }

        }
    }
}
