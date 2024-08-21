macro_rules! rrp_command {
    ($ep:ident, $timeout:expr, normal normal) => {
        #[tauri::command]
        #[specta::specta]
        async fn $ep(
            state: tauri::State<'_, State>,
            req: $ep::Request,
        ) -> Result<$ep::Response, String> {
            let ConnectedState::Connected { client } = &mut *state.0.write().await else {
                return Err("Not connected".to_string());
            };

            let res = timeout(Duration::from_millis($timeout), client.$ep(req))
                .await
                .map_err(|e| format!("Timeout: {}", e))?
                .map_err(|e| e.to_string())?;

            Ok(res)
        }
    };
}

pub(crate) use rrp_command;
