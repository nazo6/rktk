use embedded_io_async::ErrorKind;
use futures::{Stream, StreamExt as _};
use rktk_keymanager::state::State;
use rktk_rrp::{endpoints::*, server::ServerHandlers, ReadTransport, WriteTransport};

use crate::{
    config::{static_config::KEYBOARD, storage_config::StorageConfigManager},
    interface::{reporter::ReporterDriver, storage::StorageDriver},
};

use super::{ThreadModeMutex, RKTK_CONFIG};

type ConfiguredState = State<
    { RKTK_CONFIG.layer_count as usize },
    { KEYBOARD.rows as usize },
    { KEYBOARD.cols as usize },
>;

pub struct Handlers<'a, S: StorageDriver> {
    pub state: &'a ThreadModeMutex<ConfiguredState>,
    pub storage: Option<&'a StorageConfigManager<S>>,
}
impl<S: StorageDriver> ServerHandlers for Handlers<'_, S> {
    type Error = &'static str;

    async fn get_keyboard_info(
        &mut self,
        _req: (),
    ) -> Result<get_keyboard_info::Response, Self::Error> {
        Ok(get_keyboard_info::Response {
            name: heapless::String::from(KEYBOARD.name),
            cols: KEYBOARD.cols,
            rows: KEYBOARD.rows,
            keymap: ConfiguredState::get_keymap_info(),
        })
    }

    async fn get_layout_json(
        &mut self,
        _req: (),
    ) -> Result<impl Stream<Item = get_layout_json::Response>, Self::Error> {
        Ok(futures::stream::iter(
            KEYBOARD.layout.as_bytes().chunks(64).map(|chunk| {
                let mut vec = heapless::Vec::new();
                vec.extend_from_slice(chunk).unwrap();
                vec
            }),
        ))
    }

    async fn get_keymaps(
        &mut self,
        req: (),
    ) -> Result<impl Stream<Item = get_keymaps::Response>, Self::Error> {
        let keymap = self.state.lock().await.get_keymap().clone();
        Ok(futures::stream::iter(
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
        ))
    }

    async fn set_keymaps(
        &mut self,
        req: impl Stream<Item = set_keymaps::Request>,
    ) -> Result<set_keymaps::Response, Self::Error> {
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

        Ok(())
    }

    async fn get_keymap_config(
        &mut self,
        _req: get_keymap_config::Request,
    ) -> Result<get_keymap_config::Response, Self::Error> {
        Ok(self.state.lock().await.get_config().clone())
    }

    async fn set_keymap_config(
        &mut self,
        req: set_keymap_config::Request,
    ) -> Result<set_keymap_config::Response, Self::Error> {
        let keymap = self.state.lock().await.get_keymap().clone();

        if let Some(storage) = self.storage {
            if let Err(_e) = storage.write_state_config(&req).await {
                crate::print!("set_keymap_config failed");
            }
        }
        *self.state.lock().await = State::new(keymap, req);
        Ok(())
    }

    async fn get_log(
        &mut self,
        _req: get_log::Request,
    ) -> Result<impl Stream<Item = get_log::Response>, Self::Error> {
        Ok(futures::stream::iter(core::iter::from_fn(|| {
            if let Ok(chunk) = crate::task::logger::LOG_CHANNEL.try_receive() {
                crate::print!("{:?}", chunk);
                Some(chunk)
            } else {
                None
            }
        })))
    }

    async fn get_now(&mut self, _req: get_now::Request) -> Result<get_now::Response, Self::Error> {
        Ok(embassy_time::Instant::now().as_millis())
    }
}

pub struct ServerTransport<'a, R: ReporterDriver> {
    reporter: &'a R,
}

impl<'a, R: ReporterDriver> ServerTransport<'a, R> {
    pub fn new(reporter: &'a R) -> Self {
        Self { reporter }
    }
}

impl<R: ReporterDriver> ReadTransport for ServerTransport<'_, R> {
    type Error = &'static str;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.reporter
            .read_rrp_data(buf)
            .await
            .map_err(|_| "Read failed")
    }
}
impl<R: ReporterDriver> WriteTransport for ServerTransport<'_, R> {
    type Error = &'static str;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.reporter
            .send_rrp_data(buf)
            .await
            .map_err(|_| "Write failed")?;
        Ok(buf.len())
    }
}
