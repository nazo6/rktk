use dioxus::prelude::*;

use crate::app::{
    components::notification::{push_notification, Notification, NotificationLevel},
    state::CONN,
};

#[component]
pub fn Connect() -> Element {
    let window = web_sys::window().expect("Missing Window");
    let hid = window.navigator().hid();

    rsx! {
        div { class: "flex flex-col items-center justify-center h-full",
            h1 { "Connect to RKTK" }
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    let hid = hid.clone();
                    spawn(async move {
                        match conn::connect(hid.clone()).await {
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
                                    message: format!("Cannot connect to device: {:?}", e),
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
    use js_sys::wasm_bindgen::{JsCast as _, JsValue};
    use rktk_rrp_client_webhid::Client;
    use web_sys::{Hid, HidDevice};

    use crate::app::state::ConnectedState;

    #[allow(non_snake_case)]
    #[derive(serde::Serialize)]
    struct Filter {
        #[serde(rename = "usagePage")]
        pub usage_page: Option<u16>,
        pub usage: Option<u16>,
    }

    pub async fn connect(hid: Hid) -> anyhow::Result<ConnectedState> {
        let devices_promise = hid.request_device(&web_sys::HidDeviceRequestOptions::new(
            &serde_wasm_bindgen::to_value(&[Filter {
                usage_page: Some(0xFF70),
                usage: Some(0x71),
            }])
            .unwrap(),
        ));
        let devices = wasm_bindgen_futures::JsFuture::from(devices_promise)
            .await
            .map_err(|e| anyhow::anyhow!("Cannot get devices: {:?}", e))?;
        let devs_array = devices
            .dyn_ref::<js_sys::Array>()
            .context("Cannot get devices")?;
        let device: JsValue = devs_array.at(0);
        let device: HidDevice = device
            .dyn_into()
            .map_err(|_| anyhow::anyhow!("No device selected"))?;
        wasm_bindgen_futures::JsFuture::from(device.open())
            .await
            .map_err(|e| anyhow::anyhow!("Cannot open device: {:?}", e))?;

        let client = Client::new(&device);

        let keyboard = client
            .client
            .lock()
            .await
            .get_keyboard_info(())
            .await
            .context("Cannot get keyboard info")?;

        Ok(ConnectedState {
            client,
            device,
            keyboard,
        })
    }
}
