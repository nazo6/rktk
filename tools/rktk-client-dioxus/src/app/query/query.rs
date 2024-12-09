use anyhow::Context as _;
use dioxus::signals::Readable as _;
use dioxus_query::prelude::*;
use futures::TryStreamExt as _;
use kle_serial::Keyboard;
use rktk_rrp::endpoints::KeyActionLoc;

use crate::app::state::CONN;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum QueryKey {
    GetKeymap,
}

#[derive(Debug)]
pub enum QueryError {
    Anyhow(anyhow::Error),
}

#[derive(PartialEq, Debug)]
pub enum QueryValue {
    Keymap(Keyboard, Vec<KeyActionLoc>),
}

#[derive(serde::Deserialize)]
struct LayoutJson {
    keymap: Keyboard,
}

pub fn use_app_get_query<const N: usize>(
    keys: [QueryKey; N],
) -> UseQuery<QueryValue, QueryError, QueryKey> {
    use_get_query::<QueryValue, QueryError, QueryKey, _, _, N>(keys, query)
}

pub async fn query(query: Vec<QueryKey>) -> QueryResult<QueryValue, QueryError> {
    match query.first() {
        Some(QueryKey::GetKeymap) => match get_keymap().await {
            Ok((o, t)) => QueryResult::Ok(QueryValue::Keymap(o, t)),
            Err(e) => QueryResult::Err(QueryError::Anyhow(e).into()),
        },
        None => QueryResult::Err(QueryError::Anyhow(anyhow::anyhow!("Empty query"))),
    }
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
