use futures::stream::StreamExt as _;

use log::info;
use rktk_rrp::endpoints::*;
use wasm_bindgen::prelude::*;
use web_sys::SerialPort;

mod client;

#[wasm_bindgen(start)]
pub fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    info!("Hello, world from rust");
}

#[wasm_bindgen]
pub struct Client {
    serial_client: client::SerialClient,
}

#[derive(serde::Serialize, serde::Deserialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct VecGetKeymapsStreamResponse(pub Vec<KeyActionLoc>);

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(serial_port: SerialPort) -> Self {
        Client {
            serial_client: client::SerialClient {
                stream: serial_port,
            },
        }
    }

    #[wasm_bindgen]
    pub async fn get_keyboard_info(&mut self) -> Result<get_keyboard_info::Response, String> {
        self.serial_client
            .get_keyboard_info(())
            .await
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub async fn get_keymaps(&mut self) -> Result<VecGetKeymapsStreamResponse, String> {
        let stream = self
            .serial_client
            .get_keymaps(())
            .await
            .map_err(|e| format!("{:?}", e))?;
        Ok(VecGetKeymapsStreamResponse(
            stream.collect::<Vec<_>>().await,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_layout_json(&mut self) -> Result<String, String> {
        let stream = self
            .serial_client
            .get_layout_json(())
            .await
            .map_err(|e| format!("{:?}", e))?;

        let res = stream.collect::<Vec<_>>().await;
        let string =
            String::from_utf8(res.into_iter().flatten().collect()).map_err(|e| e.to_string())?;
        Ok(string)
    }

    #[wasm_bindgen]
    pub async fn set_keymaps(
        &mut self,
        keymaps: Vec<set_keymaps::StreamRequest>,
    ) -> Result<(), String> {
        self.serial_client
            .set_keymaps(futures::stream::iter(keymaps.into_iter()))
            .await
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn get_keymap_config(&mut self) -> Result<get_keymap_config::Response, String> {
        self.serial_client
            .get_keymap_config(())
            .await
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub async fn set_keymap_config(
        &mut self,
        keymap_config: set_keymap_config::Request,
    ) -> Result<set_keymap_config::Response, String> {
        self.serial_client
            .set_keymap_config(keymap_config)
            .await
            .map_err(|e| format!("{:?}", e))
    }
}
