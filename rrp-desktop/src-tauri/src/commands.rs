use futures::StreamExt as _;
use rktk_rrp::endpoints::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use tokio::sync::RwLock;

pub enum ConnectedState {
    Connected { client: super::rrp_client::Client },
    Disconnected,
}

pub struct State(RwLock<ConnectedState>);

impl State {
    pub fn new() -> Self {
        State(RwLock::new(ConnectedState::Disconnected))
    }
}

#[tauri::command]
#[specta::specta]
async fn get_serial_ports() -> Result<Vec<String>, String> {
    let ports = tokio_serial::available_ports().map_err(|e| e.to_string())?;
    let ports = ports
        .into_iter()
        .map(|port| port.port_name)
        .collect::<Vec<_>>();
    Ok(ports)
}

#[tauri::command]
#[specta::specta]
async fn connect(
    app: tauri::AppHandle,
    state: tauri::State<'_, State>,
    name: &str,
) -> Result<(), String> {
    let mut state = state.0.write().await;
    if let ConnectedState::Connected { .. } = &*state {
        return Err("Already connected".to_string());
    }

    *state = ConnectedState::Connected {
        client: super::rrp_client::Client::new(name, 115200).map_err(|e| e.to_string())?,
    };

    let _ = ConnectionEvent(true).emit(&app);

    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn disconnect(app: tauri::AppHandle, state: tauri::State<'_, State>) -> Result<(), String> {
    let mut state = state.0.write().await;
    if let ConnectedState::Disconnected = &*state {
        return Err("Already disconnected".to_string());
    }
    *state = ConnectedState::Disconnected;
    let _ = ConnectionEvent(false).emit(&app);
    Ok(())
}

#[tauri::command]
#[specta::specta]
async fn get_keyboard_info(
    state: tauri::State<'_, State>,
) -> Result<get_keyboard_info::Response, String> {
    let ConnectedState::Connected { client } = &mut *state.0.write().await else {
        return Err("Not connected".to_string());
    };

    let res = client
        .get_keyboard_info(())
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
async fn get_layout_json(state: tauri::State<'_, State>) -> Result<String, String> {
    let ConnectedState::Connected { client } = &mut *state.0.write().await else {
        return Err("Not connected".to_string());
    };

    let res = client
        .get_layout_json(())
        .await
        .map_err(|e| e.to_string())?;
    let res = res.collect::<Vec<_>>().await;
    let string =
        String::from_utf8(res.into_iter().flatten().collect()).map_err(|e| e.to_string())?;
    Ok(string)
}

#[tauri::command]
#[specta::specta]
async fn get_keymaps(
    state: tauri::State<'_, State>,
) -> Result<Vec<get_keymaps::StreamResponse>, String> {
    let ConnectedState::Connected { client } = &mut *state.0.write().await else {
        return Err("Not connected".to_string());
    };

    let res = client.get_keymaps(()).await.map_err(|e| e.to_string())?;
    let res = res.collect().await;
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ConnectionEvent(bool);

type TauriSpectaBuilder = tauri_specta::Builder<tauri::Wry>;
pub fn tauri_specta_builder() -> TauriSpectaBuilder {
    let builder = TauriSpectaBuilder::new()
        .events(tauri_specta::collect_events![ConnectionEvent,])
        .commands(tauri_specta::collect_commands![
            connect,
            disconnect,
            get_serial_ports,
            get_keyboard_info,
            get_keymaps,
            get_layout_json,
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    builder
}
