use dioxus::prelude::*;
use dioxus_elements::geometry::PixelsSize;

use super::Layer;

const SIZE_AMP: f64 = 60.0;

#[component]
pub fn Keyboard(keymap: Layer) -> Element {
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
                    KeyboardInner { keymap }
                }
            }
        }
    }
}

#[component]
pub fn KeyboardInner(keymap: Layer) -> Element {
    rsx! {
        for key in keymap.iter().flatten() {
            if let Some(key) = key.key.as_ref() {
                Key { kle_key: key.clone() }
            }
        }
    }
}

#[component]
pub fn Key(kle_key: kle_serial::Key) -> Element {
    rsx! {
        div {
            class: "absolute border-2 p-1 font-bold cursor-pointer hover:bg-gray-500/20 overflow-hidden",
            width: format!("{}px", kle_key.width * SIZE_AMP),
            height: format!("{}px", kle_key.height * SIZE_AMP),
            top: format!("{}px", kle_key.y * SIZE_AMP),
            left: format!("{}px", kle_key.x * SIZE_AMP),
            transform: format!("rotate({}deg)", kle_key.rotation),
        }
    }
}
