use std::collections::HashMap;

use anyhow::Context as _;
use dioxus::signals::ReadableExt as _;
use futures::TryStreamExt as _;
use kle_serial::Keyboard;
use kmsm::keycode::KeyAction;
use rktk_rrp::endpoints::{KeyActionLoc, get_keyboard_info::KeyboardInfo};

use crate::{app::state::CONN, backend::RrpHidDevice as _};

#[derive(Clone, PartialEq, Debug)]
pub struct KeyData {
    pub key: Option<kle_serial::Key>,
    pub action: Option<KeyAction>,
}

pub type KeymapData = Vec<Vec<Vec<KeyData>>>;

#[derive(serde::Deserialize)]
struct LayoutJson {
    keymap: Keyboard,
}

pub async fn get_keymap() -> anyhow::Result<KeymapData> {
    let conn = &*CONN.read();
    let conn = conn.as_ref().context("Not connected")?;
    let mut be = conn.device.lock().await;
    let client = be.get_client();

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

    Ok(process_keymap(
        conn.keyboard.clone(),
        layout.keymap,
        keymaps,
    ))
}

fn get_keymap_mut(keymap: &mut KeymapData, layer: u8, row: u8, col: u8) -> Option<&mut KeyData> {
    keymap
        .get_mut(layer as usize)
        .and_then(|layer| layer.get_mut(row as usize))
        .and_then(|row| row.get_mut(col as usize))
}

fn process_keymap(
    keyboard: KeyboardInfo,
    layout: kle_serial::Keyboard,
    key_data: Vec<KeyActionLoc>,
) -> KeymapData {
    let mut keymap: KeymapData = vec![
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
    keymap
}

pub async fn set_keymap(changes: &HashMap<(u8, u8, u8), KeyAction>) -> anyhow::Result<()> {
    let conn = &*CONN.read();
    let conn = conn.as_ref().context("Not connected")?;
    let mut d = conn.device.lock().await;
    let client = d.get_client();

    let stream =
        futures::stream::iter(changes.iter().map(|((layer, row, col), key)| KeyActionLoc {
            layer: *layer,
            row: *row,
            col: *col,
            key: *key,
        }));

    client.set_keymaps(stream).await?;

    Ok(())
}
