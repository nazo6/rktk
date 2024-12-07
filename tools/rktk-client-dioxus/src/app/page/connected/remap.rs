use dioxus::prelude::*;
use rktk_rrp::endpoints::{
    get_keyboard_info::KeyboardInfo, rktk_keymanager::keycode::KeyAction, KeyActionLoc,
};

use crate::app::state::CONN;

mod keyboard;

#[component]
pub fn Remap() -> Element {
    let keymaps = use_resource(fetcher::get_keymap);

    let keymaps = keymaps.read();

    let keyboard = CONN
        .read()
        .as_ref()
        .context("Not connected")?
        .keyboard
        .clone();

    match &*keymaps {
        Some(Ok((layout, keymap))) => {
            rsx! {
                div { class: "h-full",
                    RemapInner {
                        keyboard,
                        layout: layout.clone(),
                        key_data: keymap.clone(),
                    }
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

#[derive(Clone, PartialEq)]
struct KeyData {
    key: Option<kle_serial::Key>,
    action: Option<KeyAction>,
}

type Layer = Vec<Vec<KeyData>>;
type Keymap = Vec<Vec<Vec<KeyData>>>;

fn get_keymap_mut(keymap: &mut Keymap, layer: u8, row: u8, col: u8) -> Option<&mut KeyData> {
    keymap
        .get_mut(layer as usize)
        .and_then(|layer| layer.get_mut(row as usize))
        .and_then(|row| row.get_mut(col as usize))
}

#[component]
pub fn RemapInner(
    keyboard: KeyboardInfo,
    layout: kle_serial::Keyboard,
    key_data: Vec<KeyActionLoc>,
) -> Element {
    let mut keymap: Keymap = vec![
        vec![
            vec![
                KeyData {
                    key: None,
                    action: None,
                };
                keyboard.cols as usize
            ];
            keyboard.rows as usize
        ];
        keyboard.keymap.layer_count as usize
    ];

    for key in key_data {
        if let Some(key_data) = get_keymap_mut(&mut keymap, key.layer, key.row, key.col) {
            key_data.action = Some(key.key);
        }
    }

    for key_layout in layout.keys.iter() {
        if let Some(legend) = &key_layout.legends[0] {
            let Some(split) = legend.text.split_once(",") else {
                continue;
            };
            let Ok(row) = split.0.parse::<usize>() else {
                continue;
            };
            let Ok(col) = split.1.parse::<usize>() else {
                continue;
            };

            for layer in 0..keyboard.keymap.layer_count {
                if let Some(key_data) = get_keymap_mut(&mut keymap, layer, row as u8, col as u8) {
                    key_data.key = Some(key_layout.clone());
                }
            }
        }
    }

    rsx! {
        div { class: "h-full flex justify-center pt-12",
            keyboard::Keyboard { keymap: keymap[0].clone() }
        }
    }
}

mod fetcher {
    use anyhow::Context as _;
    use dioxus::signals::Readable as _;
    use futures::TryStreamExt as _;
    use kle_serial::Keyboard;
    use rktk_rrp::endpoints::KeyActionLoc;

    use crate::app::state::CONN;

    #[derive(serde::Deserialize)]
    struct LayoutJson {
        keymap: Keyboard,
    }

    pub async fn get_keymap() -> anyhow::Result<(Keyboard, Vec<KeyActionLoc>)> {
        let conn = &*CONN.read();
        let conn = conn.as_ref().context("Not connected")?;
        let mut client = conn.client.client.lock().await;

        let json = client.get_layout_json(()).await?;
        let json = json
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        let json_str = std::str::from_utf8(&json[..]).context("Invalid UTF-8")?;
        let layout: LayoutJson = serde_json::from_str(json_str).context("Invalid JSON")?;

        let keymaps = client.get_keymaps(()).await?;
        let keymaps = keymaps.try_collect::<Vec<_>>().await?;

        Ok((layout.keymap, keymaps))
    }
}
