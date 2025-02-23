use dioxus::prelude::*;

use crate::backend::RrpHidDevice as _;

use super::{
    components::notification::{push_notification, Notification, NotificationLevel},
    state::CONN,
};

pub async fn disconnect() -> anyhow::Result<()> {
    {
        let Some(state) = &*CONN.read() else {
            return Err(anyhow::anyhow!("State is None"));
        };
        state
            .device
            .lock()
            .await
            .close()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to close device: {:?}", e))?;
    }

    push_notification(Notification {
        message: "Disconnected from device".to_string(),
        level: NotificationLevel::Info,
        ..Default::default()
    });

    *CONN.write() = None;

    Ok(())
}
