use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_elements::geometry::PixelsSize;
use rktk_rrp::endpoints::rktk_keymanager::keycode::KeyAction;

use super::KeymapData;

const SIZE_AMP: f64 = 60.0;

#[component]
pub fn Keyboard(
    keymap: KeymapData,
    layer: Signal<usize>,
    select_signal: Signal<Option<(usize, usize)>>,
    keymap_changes: ReadOnlySignal<HashMap<(u8, u8, u8), KeyAction>>,
) -> Element {
    let keyboard_width = keymap
        .iter()
        .flatten()
        .flatten()
        .fold(0 as f64, |max, key| {
            key.key
                .as_ref()
                .map(|k| k.x + k.width)
                .unwrap_or(0 as f64)
                .max(max)
        })
        * SIZE_AMP
        + 10.0;
    let keyboard_height = keymap
        .iter()
        .flatten()
        .flatten()
        .fold(0 as f64, |max, key| {
            key.key
                .as_ref()
                .map(|k| k.y + k.height)
                .unwrap_or(0 as f64)
                .max(max)
        })
        * SIZE_AMP
        + 10.0;

    let mut elem_size = use_signal(|| Option::<PixelsSize>::None);

    let scale = if let Some(size) = &*elem_size.read() {
        (size.width - 150.0) / keyboard_width
    } else {
        1.0
    };

    rsx! {
        div {
            class: "w-full flex justify-center max-w-[80rem]",
            onresize: move |evt| { elem_size.set(evt.data().get_content_box_size().ok()) },
            div {
                width: format!("{}px", keyboard_width * scale),
                height: format!("{}px", keyboard_height * scale),
                div { class: "flex gap-2 h-full",
                    div { class: "flex flex-col gap-2 justify-center pr-2",
                        for (l , _) in keymap.iter().enumerate() {
                            button {
                                class: "btn btn-square btn-sm rounded-sm",
                                class: if *layer.read() == l { "btn-primary" },
                                onclick: move |_| layer.set(l),
                                "{l}"
                            }
                        }
                    }
                    div {
                        class: "relative h-auto w-auto",
                        transform: format!("scale({})", scale),
                        transform_origin: "top left",
                        for (row , key) in keymap[*layer.read()].iter().enumerate() {
                            for (col , key) in key.iter().enumerate() {
                                if let (Some(key), Some(action)) = (key.key.as_ref(), key.action.as_ref()) {
                                    Key {
                                        kle_key: key.clone(),
                                        action: *action,
                                        row,
                                        col,
                                        select_signal,
                                        changed: keymap_changes.read().contains_key(&(*layer.read() as u8, row as u8, col as u8)),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Key(
    kle_key: kle_serial::Key,
    action: KeyAction,
    row: usize,
    col: usize,
    mut select_signal: Signal<Option<(usize, usize)>>,
    changed: bool,
) -> Element {
    rsx! {
        div {
            onclick: move |_| { select_signal.set(Some((row, col))) },
            class: "absolute border-2 p-1 font-bold cursor-pointer hover:bg-gray-500/20 overflow-hidden text-xs",
            class: if Some((row, col)) == *select_signal.read() { "border-accent" } else { "border-base-content" },
            class: if changed { "text-red-500" },
            width: format!("{}px", kle_key.width * SIZE_AMP - 2.0),
            height: format!("{}px", kle_key.height * SIZE_AMP - 2.0),
            top: format!("{}px", kle_key.y * SIZE_AMP),
            left: format!("{}px", kle_key.x * SIZE_AMP),
            transform: format!("rotate({}deg)", kle_key.rotation),
            {utils::key_str(&action)}
        }
    }
}

mod utils {
    use rktk_keymanager::keycode::{layer::LayerOp, KeyAction, KeyCode};

    pub fn key_str(key: &KeyAction) -> String {
        match key {
            KeyAction::Inherit => "___".to_string(),
            KeyAction::Normal(key_code) => keycode_str(key_code),
            KeyAction::Normal2(key_code, key_code1) => {
                format!("{} & {}", keycode_str(key_code), keycode_str(key_code1))
            }
            KeyAction::TapHold(key_code, key_code1) => {
                format!("{} / {}", keycode_str(key_code), keycode_str(key_code1))
            }
            KeyAction::OneShot(key_code) => format!("OS({})", keycode_str(key_code)),
            KeyAction::TapDance(id) => format!("TD({})", id),
        }
    }

    fn keycode_str(key: &KeyCode) -> String {
        match key {
            KeyCode::None => "XXX".to_string(),
            KeyCode::Key(key) => Into::<&'static str>::into(key).to_string(),
            KeyCode::Mouse(mouse) => format!("{}", mouse),
            KeyCode::Modifier(modifier) => format!("{}", modifier),
            KeyCode::Layer(layer_op) => match layer_op {
                LayerOp::Momentary(l) => format!("MO({})", l),
                LayerOp::Toggle(l) => format!("TO({})", l),
            },
            KeyCode::Special(special) => Into::<&'static str>::into(special).to_string(),
            KeyCode::Media(media) => Into::<&'static str>::into(media).to_string(),
        }
    }
}
