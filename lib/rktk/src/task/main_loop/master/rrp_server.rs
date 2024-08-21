use futures::{Stream, StreamExt as _};
use get_keyboard_info::KeyboardInfo;
use rktk_keymanager::state::State;
use rktk_rrp::{endpoint_server, endpoints::*, server::EndpointTransport};

use crate::{
    config::static_config::CONFIG,
    interface::{error::RktkError, reporter::ReporterDriver},
    utils::ThreadModeMutex,
};

pub struct EndpointTransportImpl<'a, R: ReporterDriver>(pub &'a R);

impl<'a, R: ReporterDriver> EndpointTransport for EndpointTransportImpl<'a, R> {
    type Error = RktkError;
    async fn read_until_zero(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut reader = [0];
        let mut read = 0;
        loop {
            let Ok(crr_read) = self.0.read_rrp_data(&mut reader).await else {
                continue;
            };
            if crr_read == 0 {
                continue;
            }

            if let Some(byte) = buf.get_mut(read) {
                *byte = reader[0];
            } else {
                crate::print!("Invalid byte received");

                return Err(RktkError::GeneralError("Invalid byte received"));
            }

            read += crr_read;

            if reader[0] == 0 {
                break;
            }
        }

        Ok(read)
    }
    async fn send_all(&self, buf: &[u8]) -> Result<(), Self::Error> {
        self.0.send_rrp_data(buf).await
    }
}

type ConfiguredState = State<{ CONFIG.layer_count }, { CONFIG.rows }, { CONFIG.cols }>;

pub struct Server<'a> {
    pub state: &'a ThreadModeMutex<ConfiguredState>,
}
impl<'a> Server<'a> {
    endpoint_server!(
        get_keyboard_info normal normal => get_info
        get_layout_json normal stream => get_layout_json
        get_keymaps normal stream => get_keymaps
        set_keymaps stream normal => set_keymaps
        get_keymap_config normal normal => get_keymap_config
        set_keymap_config normal normal => set_keymap_config
    );

    async fn get_info(&mut self, _req: get_keyboard_info::Request) -> get_keyboard_info::Response {
        KeyboardInfo {
            name: heapless::String::from(CONFIG.name),
            cols: CONFIG.cols as u8,
            rows: CONFIG.rows as u8,
            keymap: ConfiguredState::get_keymap_info(),
        }
    }
    async fn get_keymaps(
        &mut self,
        _req: get_keymaps::Request,
    ) -> impl Stream<Item = get_keymaps::StreamResponse> + '_ {
        let keymap = self.state.lock().await.get_keymap().clone();
        futures::stream::iter(
            itertools::iproduct!(0..CONFIG.layer_count, 0..CONFIG.rows, 0..CONFIG.cols).map(
                move |(layer, row, col)| KeyActionLoc {
                    layer: layer as u8,
                    row: row as u8,
                    col: col as u8,
                    key: keymap[layer].map[row][col],
                },
            ),
        )
    }

    async fn get_layout_json(
        &mut self,
        _req: get_layout_json::Request,
    ) -> impl Stream<Item = get_layout_json::StreamResponse> + '_ {
        futures::stream::iter(CONFIG.layout_json.as_bytes().chunks(64).map(|chunk| {
            let mut vec = heapless::Vec::new();
            vec.extend_from_slice(chunk).unwrap();
            vec
        }))
    }

    async fn set_keymaps(
        &mut self,
        req: impl Stream<Item = set_keymaps::StreamRequest>,
    ) -> set_keymaps::Response {
        let mut req = core::pin::pin!(req);

        let (mut keymap, config) = {
            let state = self.state.lock().await;
            (state.get_keymap().clone(), state.get_config().clone())
        };

        while let Some(key) = req.next().await {
            keymap[key.layer as usize].map[key.row as usize][key.col as usize] = key.key;
        }
        *self.state.lock().await = State::new(keymap, config);
    }

    async fn get_keymap_config(
        &mut self,
        _req: get_keymap_config::Request,
    ) -> get_keymap_config::Response {
        self.state.lock().await.get_config().clone()
    }

    async fn set_keymap_config(
        &mut self,
        req: set_keymap_config::Request,
    ) -> set_keymap_config::Response {
        let keymap = self.state.lock().await.get_keymap().clone();

        *self.state.lock().await = State::new(keymap, req);
    }
}
