use dioxus_query::prelude::*;

pub mod get_keymap;

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
    Keymap(get_keymap::KeymapData),
}

pub fn use_app_get_query<const N: usize>(
    keys: [QueryKey; N],
) -> UseQuery<QueryValue, QueryError, QueryKey> {
    use_get_query::<QueryValue, QueryError, QueryKey, _, _, N>(keys, query)
}

pub async fn query(query: Vec<QueryKey>) -> QueryResult<QueryValue, QueryError> {
    match query.first() {
        Some(QueryKey::GetKeymap) => match get_keymap::get_keymap().await {
            Ok(res) => QueryResult::Ok(QueryValue::Keymap(res)),
            Err(e) => QueryResult::Err(QueryError::Anyhow(e)),
        },
        None => QueryResult::Err(QueryError::Anyhow(anyhow::anyhow!("Empty query"))),
    }
}
