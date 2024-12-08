use dioxus::prelude::*;
use dioxus_elements::geometry::PixelsSize;
use rktk_rrp::endpoints::rktk_keymanager::keycode::KeyAction;

use super::Layer;

const SIZE_AMP: f64 = 60.0;

#[component]
pub fn Keyboard(keymap: Layer, select_signal: Signal<Option<(usize, usize)>>) -> Element {
    let keyboard_width = keymap.iter().flatten().fold(0 as f64, |max, key| {
        key.key
            .as_ref()
            .map(|k| k.x + k.width)
            .unwrap_or(0 as f64)
            .max(max)
    }) * SIZE_AMP;
    let keyboard_height = keymap.iter().flatten().fold(0 as f64, |max, key| {
        key.key
            .as_ref()
            .map(|k| k.y + k.height)
            .unwrap_or(0 as f64)
            .max(max)
    }) * SIZE_AMP;

    let mut elem_size = use_signal(|| Option::<PixelsSize>::None);

    let scale = if let Some(size) = &*elem_size.read() {
        ((size.width - 150.0) / keyboard_width).min(size.height / keyboard_height)
    } else {
        1.0
    };

    rsx! {
        div {
            class: "h-full w-full max-w-[80rem] flex justify-center",
            onresize: move |evt| elem_size.set(evt.data().get_content_box_size().ok()),
            div {
                width: format!("{}px", keyboard_width * scale),
                height: format!("{}px", keyboard_height * scale),
                div {
                    class: "relative h-auto w-auto",
                    transform: format!("scale({})", scale),
                    transform_origin: "top left",
                    for (row , key) in keymap.iter().enumerate() {
                        for (col , key) in key.iter().enumerate() {
                            if let (Some(key), Some(action)) = (key.key.as_ref(), key.action.as_ref()) {
                                Key {
                                    kle_key: key.clone(),
                                    action: *action,
                                    row,
                                    col,
                                    select_signal,
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
) -> Element {
    rsx! {
        div {
            onclick: move |_| { select_signal.set(Some((row, col))) },
            class: "absolute border-2 p-1 font-bold cursor-pointer hover:bg-gray-500/20 overflow-hidden text-xs",
            class: if Some((row, col)) == *select_signal.read() { "border-accent" } else { "border-primary-content" },
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