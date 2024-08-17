#![allow(clippy::new_without_default)]

pub mod commands;
mod rrp_client;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tsb = commands::tauri_specta_builder();

    tauri::Builder::default()
        // and finally tell Tauri how to invoke them
        .invoke_handler(tsb.invoke_handler())
        .manage(commands::State::new())
        .setup(move |app| {
            tsb.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
