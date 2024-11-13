use futures::{Stream, StreamExt as _};
use get_keyboard_info::KeyboardInfo;
use rktk_keymanager::state::State;
use rktk_rrp::{endpoint_server, endpoints::*, server::EndpointTransport};

use crate::{
    config::{
        static_config::{KEYBOARD, RKTK_CONFIG},
        storage_config::StorageConfigManager,
    },
    interface::{error::RktkError, reporter::ReporterDriver, storage::StorageDriver},
    utils::ThreadModeMutex,
};

pub struct EndpointTransportImpl<'a, R: ReporterDriver>(pub &'a R);

impl<R: ReporterDriver> EndpointTransport for EndpointTransportImpl<'_, R> {
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
                log::warn!("Invalid byte received");

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

type ConfiguredState = State<
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
>;

pub struct Server<'a, S: StorageDriver> {
    pub state: &'a ThreadModeMutex<ConfiguredState>,
    pub storage: Option<&'a StorageConfigManager<S>>,
}
impl<S: StorageDriver> Server<'_, S> {
    endpoint_server!(
        get_keyboard_info normal normal => get_info
        get_layout_json normal stream => get_layout_json
        get_keymaps normal stream => get_keymaps
        set_keymaps stream normal => set_keymaps
        get_keymap_config normal normal => get_keymap_config
        set_keymap_config normal normal => set_keymap_config
        get_log normal stream => get_log
        get_now normal normal => get_now
    );

    async fn get_info(&mut self, _req: get_keyboard_info::Request) -> get_keyboard_info::Response {
        KeyboardInfo {
            name: heapless::String::from(KEYBOARD.name),
            cols: KEYBOARD.cols,
            rows: KEYBOARD.rows,
            keymap: ConfiguredState::get_keymap_info(),
        }
    }
    async fn get_keymaps(
        &mut self,
        _req: get_keymaps::Request,
    ) -> impl Stream<Item = get_keymaps::StreamResponse> + '_ {
        let keymap = self.state.lock().await.get_keymap().clone();
        futures::stream::iter(
            itertools::iproduct!(
                0..RKTK_CONFIG.layer_count,
                0..KEYBOARD.rows,
                0..KEYBOARD.cols
            )
            .map(move |(layer, row, col)| KeyActionLoc {
                layer,
                row,
                col,
                key: keymap[layer as usize].map[row as usize][col as usize],
            }),
        )
    }

    async fn get_layout_json(
        &mut self,
        _req: get_layout_json::Request,
    ) -> impl Stream<Item = get_layout_json::StreamResponse> + '_ {
        futures::stream::iter(KEYBOARD.layout.as_bytes().chunks(64).map(|chunk| {
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
            if let Some(storage) = self.storage {
                if let Err(_e) = storage
                    .write_keymap(key.layer, &keymap[key.layer as usize])
                    .await
                {
                    crate::print!("set_keymaps failed");
                }
            }
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

        if let Some(storage) = self.storage {
            if let Err(_e) = storage.write_state_config(&req).await {
                crate::print!("set_keymap_config failed");
            }
        }
        *self.state.lock().await = State::new(keymap, req);
    }

    async fn get_log(
        &mut self,
        _req: get_log::Request,
    ) -> impl Stream<Item = get_log::StreamResponse> + '_ {
        futures::stream::iter(core::iter::from_fn(|| {
            if let Ok(chunk) = crate::task::logger::LOG_CHANNEL.try_receive() {
                Some(chunk)
            } else {
                None
            }
        }))
    }

    async fn get_now(&mut self, _req: get_now::Request) -> get_now::Response {
        embassy_time::Instant::now().as_millis()
    }
}
