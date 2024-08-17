use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

pub mod commands;
mod rrp_client;

struct State {
    client: rrp_client::Client,
}

#[tauri::command]
#[specta::specta]
async fn keyboard_info(
    state: tauri::State<'_, State>,
) -> Result<rktk_rrp::endpoints::get_keyboard_info::Response, String> {
    state
        .client
        .get_keyboard_info(())
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![keyboard_info,]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .manage(State {
            client: rrp_client::Client::new(),
        })
        .setup(move |app| {
            // This is also required if you want to use events
            builder.mount_events(app);

            Ok(())
        })
        // on an actual app, remove the string argument
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
