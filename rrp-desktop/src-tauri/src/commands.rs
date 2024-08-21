use std::time::Duration;

use futures::StreamExt as _;
use macros::rrp_command;
use rktk_rrp::endpoints::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use tokio::{sync::RwLock, time::timeout};

mod macros;
mod serial_ports;

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

#[derive(Serialize, Deserialize, Type)]
struct SerialPortInfoType(
    #[specta(type = serial_ports::SerialPortInfo)] tokio_serial::SerialPortInfo,
);

#[tauri::command]
#[specta::specta]
async fn get_serial_ports() -> Result<Vec<SerialPortInfoType>, String> {
    let ports = tokio_serial::available_ports().map_err(|e| e.to_string())?;
    let ports = ports.into_iter().map(SerialPortInfoType).collect();
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
        client: timeout(
            Duration::from_secs(1),
            super::rrp_client::Client::connect(name, 115200),
        )
        .await
        .map_err(|e| format!("Timeout: {}", e))?
        .map_err(|e| e.to_string())?,
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
async fn get_layout_json(state: tauri::State<'_, State>) -> Result<String, String> {
    let ConnectedState::Connected { client } = &mut *state.0.write().await else {
        return Err("Not connected".to_string());
    };

    let res = timeout(Duration::from_secs(1), client.get_layout_json(()))
        .await
        .map_err(|e| format!("Timeout: {}", e))?
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

    let res = timeout(Duration::from_secs(1), client.get_keymaps(()))
        .await
        .map_err(|e| format!("Timeout: {}", e))?
        .map_err(|e| e.to_string())?;
    let res = res.collect().await;
    Ok(res)
}

#[tauri::command]
#[specta::specta]
async fn set_keymaps(
    state: tauri::State<'_, State>,
    keymaps: Vec<set_keymaps::StreamRequest>,
) -> Result<(), String> {
    let ConnectedState::Connected { client } = &mut *state.0.write().await else {
        return Err("Not connected".to_string());
    };

    timeout(
        Duration::from_secs(1),
        client.set_keymaps(futures::stream::iter(keymaps.into_iter())),
    )
    .await
    .map_err(|e| format!("Timeout: {}", e))?
    .map_err(|e| e.to_string())?;

    Ok(())
}

rrp_command!(get_keyboard_info, 1000, normal normal);
rrp_command!(get_keymap_config, 1000, normal normal);
rrp_command!(set_keymap_config, 1000, normal normal);

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
            set_keymaps,
            get_keymap_config,
            set_keymap_config,
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
