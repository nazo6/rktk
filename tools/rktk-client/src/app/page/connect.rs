use dioxus::prelude::*;

use crate::app::{
    components::notification::{Notification, NotificationLevel, push_notification},
    state::CONN,
};

#[component]
pub fn Connect() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-full",
            h1 { "Connect to RKTK" }
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    spawn(async move {
                        match conn::connect().await {
                            Ok(state) => {
                                push_notification(Notification {
                                    message: format!("Connected to device: {}", state.keyboard.name),
                                    level: NotificationLevel::Info,
                                    ..Default::default()
                                });
                                *CONN.write() = Some(state);
                            }
                            Err(e) => {
                                push_notification(Notification {
                                    message: format!("Cannot connect to device: {e:?}"),
                                    level: NotificationLevel::Error,
                                    ..Default::default()
                                });
                            }
                        }
                    });
                },
                "Connect"
            }
        }
    }
}

mod conn {
    use anyhow::Context as _;
    use futures::lock::Mutex;

    use crate::{
        app::{BACKEND, state::ConnectedState},
        backend::{RrpHidBackend, RrpHidDevice as _},
    };

    pub async fn connect() -> anyhow::Result<ConnectedState> {
        let mut device = BACKEND.0.open_device(0xFF70, 0x71).await?;
        let keyboard = device
            .get_client()
            .get_keyboard_info(())
            .await
            .context("Cannot get keyboard info")?;

        Ok(ConnectedState {
            device: Mutex::new(device),
            keyboard,
        })
    }
}
