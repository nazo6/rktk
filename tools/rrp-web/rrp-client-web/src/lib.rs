use async_lock::Mutex;
use client::{HidReader, HidWriter};
use futures::StreamExt;

use log::info;
use rktk_rrp::endpoints::*;
use wasm_bindgen::prelude::*;
use web_sys::HidDevice;

mod client;

#[wasm_bindgen(start)]
pub fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
    info!("rrp-client-web started!");
}

#[wasm_bindgen]
pub struct Client {
    client: Mutex<rktk_rrp::client::Client<HidReader, HidWriter>>,
}

#[derive(serde::Serialize, serde::Deserialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct VecGetKeymapsStreamResponse(pub Vec<KeyActionLoc>);

#[derive(serde::Serialize, serde::Deserialize, tsify_next::Tsify, Default)]
pub struct LogEntry {
    time: u64,
    level: get_log::LogLevel,
    message: String,
    line: Option<u32>,
}
#[derive(serde::Serialize, serde::Deserialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct VecLogEntry(pub Vec<LogEntry>);

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(device: HidDevice) -> Self {
        Client {
            client: Mutex::new(rktk_rrp::client::Client::new(
                HidReader::new(device.clone()),
                HidWriter::new(device.clone()),
            )),
        }
    }

    #[wasm_bindgen]
    pub async fn get_keyboard_info(&self) -> Result<get_keyboard_info::Response, String> {
        self.client
            .lock()
            .await
            .get_keyboard_info(())
            .await
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub async fn get_keymaps(&self) -> Result<VecGetKeymapsStreamResponse, String> {
        let mut serial = self.client.lock().await;
        let stream = serial
            .get_keymaps(())
            .await
            .map_err(|e| format!("{:?}", e))?;
        Ok(VecGetKeymapsStreamResponse(
            stream.collect::<Vec<_>>().await,
        ))
    }

    #[wasm_bindgen]
    pub async fn get_layout_json(&self) -> Result<String, String> {
        let mut serial = self.client.lock().await;
        let stream = serial
            .get_layout_json(())
            .await
            .map_err(|e| format!("{:?}", e))?;

        let res = stream.collect::<Vec<_>>().await;

        log::info!("get_layout_json: {:?}", &res);

        let string =
            String::from_utf8(res.into_iter().flatten().collect()).map_err(|e| e.to_string())?;

        log::info!("get_layout_json: {:?}", &string);
        Ok(string)
    }

    #[wasm_bindgen]
    pub async fn set_keymaps(&mut self, keymaps: Vec<set_keymaps::Request>) -> Result<(), String> {
        let mut serial = self.client.lock().await;
        serial
            .set_keymaps(futures::stream::iter(keymaps.into_iter()))
            .await
            .map_err(|e| format!("{:?}", e))?;
        Ok(())
    }

    #[wasm_bindgen]
    pub async fn get_keymap_config(&self) -> Result<get_keymap_config::Response, String> {
        let mut serial = self.client.lock().await;
        serial
            .get_keymap_config(())
            .await
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub async fn set_keymap_config(
        &mut self,
        keymap_config: set_keymap_config::Request,
    ) -> Result<set_keymap_config::Response, String> {
        let mut serial = self.client.lock().await;
        serial
            .set_keymap_config(keymap_config)
            .await
            .map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub async fn get_now(&self) -> Result<u64, String> {
        let mut serial = self.client.lock().await;
        serial.get_now(()).await.map_err(|e| format!("{:?}", e))
    }

    #[wasm_bindgen]
    pub async fn get_log(&self) -> Result<VecLogEntry, String> {
        let mut serial = self.client.lock().await;
        let stream = serial.get_log(()).await.map_err(|e| format!("{:?}", e))?;
        let mut stream = std::pin::pin!(stream);

        let mut logs = Vec::new();
        let mut log = LogEntry::default();
        let mut log_bytes = Vec::new();
        while let Some(chunk) = stream.next().await {
            match chunk {
                get_log::LogChunk::Start { time, level, line } => {
                    log.time = time;
                    log.level = level;
                    log.line = line;
                }
                get_log::LogChunk::Bytes { bytes, len } => {
                    log_bytes.extend_from_slice(&bytes[..len as usize]);
                }
                get_log::LogChunk::End => {
                    let Ok(message) =
                        String::from_utf8(log_bytes.clone()).map_err(|e| e.to_string())
                    else {
                        continue;
                    };
                    log.message = message;
                    logs.push(log);

                    log = LogEntry::default();
                    log_bytes.clear();
                }
            }
        }
        Ok(VecLogEntry(logs))
    }
}
