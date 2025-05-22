use core::fmt::Display;

use futures::{Stream, StreamExt as _};
use rktk_rrp::{
    endpoints::*,
    server::ServerHandlers,
    transport::{ReadTransport, WriteTransport, error::ReceiveError},
};

use crate::{
    config::{
        constant::{CONST_CONFIG, schema::DynamicConfig},
        storage::StorageConfigManager,
    },
    drivers::interface::{
        reporter::ReporterDriver, storage::StorageDriver, usb::UsbReporterDriver,
        wireless::WirelessReporterDriver,
    },
};

use super::{ConfiguredState, SharedState};

pub async fn start(
    config: &'static DynamicConfig,
    usb: &Option<impl UsbReporterDriver>,
    _ble: &Option<impl WirelessReporterDriver>,
    state: &SharedState,
    config_store: &Option<StorageConfigManager<impl StorageDriver>>,
) {
    if let Some(usb) = &usb {
        let mut server = rktk_rrp::server::Server::<_, _, _>::new(
            ServerTransport::new(usb),
            ServerTransport::new(usb),
            Handlers {
                state,
                storage: config_store.as_ref(),
                config,
            },
        );
        server.start::<{ CONST_CONFIG.buffer.rrp }>().await;
    }
}

struct Handlers<'a, S: StorageDriver> {
    state: &'a SharedState,
    storage: Option<&'a StorageConfigManager<S>>,
    config: &'static DynamicConfig,
}
impl<RE: Display, WE: Display, S: StorageDriver> ServerHandlers<RE, WE> for Handlers<'_, S> {
    type Error = &'static str;

    async fn get_keyboard_info(
        &mut self,
        _req: (),
    ) -> Result<get_keyboard_info::Response, Self::Error> {
        Ok(get_keyboard_info::Response {
            name: heapless::String::from(self.config.keyboard.name),
            cols: CONST_CONFIG.keyboard.cols,
            rows: CONST_CONFIG.keyboard.rows,
            keymap: ConfiguredState::get_keymap_info(),
        })
    }

    async fn get_layout_json(
        &mut self,
        _req: (),
    ) -> Result<impl Stream<Item = get_layout_json::Response>, Self::Error> {
        if let Some(layout) = self.config.keyboard.layout {
            Ok(futures::stream::iter(layout.as_bytes().chunks(64).map(
                |chunk| {
                    let mut vec = heapless::Vec::new();
                    vec.extend_from_slice(chunk).unwrap();
                    vec
                },
            )))
        } else {
            Err("Layout is not defined")
        }
    }

    async fn get_keymaps(
        &mut self,
        _req: (),
    ) -> Result<impl Stream<Item = get_keymaps::Response>, Self::Error> {
        let keymap = self.state.lock().await.inner().get_keymap().clone();
        Ok(futures::stream::iter(
            itertools::iproduct!(
                0..CONST_CONFIG.key_manager.layer_count,
                0..CONST_CONFIG.keyboard.rows,
                0..CONST_CONFIG.keyboard.cols
            )
            .map(move |(layer, row, col)| KeyActionLoc {
                layer,
                row,
                col,
                key: keymap.layers[layer as usize].keymap[row as usize][col as usize],
            }),
        ))
    }

    async fn set_keymaps(
        &mut self,
        req: impl Stream<Item = Result<set_keymaps::Request, ReceiveError<RE>>>,
    ) -> Result<set_keymaps::Response, Self::Error> {
        let mut req = core::pin::pin!(req);

        let (mut keymap, config) = {
            let state = self.state.lock().await;
            (
                state.inner().get_keymap().clone(),
                state.inner().get_config().clone(),
            )
        };

        while let Some(Ok(key)) = req.next().await {
            keymap.layers[key.layer as usize].keymap[key.row as usize][key.col as usize] = key.key;
            if let Some(storage) = self.storage {
                if let Err(_e) = storage
                    .write_keymap(key.layer, &keymap.layers[key.layer as usize])
                    .await
                {
                    crate::print!("set_keymaps failed");
                }
            }
        }
        *self.state.lock().await = ConfiguredState::new(keymap, config);

        Ok(())
    }

    async fn get_keymap_config(
        &mut self,
        _req: get_keymap_config::Request,
    ) -> Result<get_keymap_config::Response, Self::Error> {
        Ok(self.state.lock().await.inner().get_config().clone())
    }

    async fn set_keymap_config(
        &mut self,
        req: set_keymap_config::Request,
    ) -> Result<set_keymap_config::Response, Self::Error> {
        let keymap = self.state.lock().await.inner().get_keymap().clone();

        if let Some(storage) = self.storage {
            if let Err(_e) = storage.write_state_config(&req).await {
                crate::print!("set_keymap_config failed");
            }
        }
        *self.state.lock().await = ConfiguredState::new(keymap, req);
        Ok(())
    }

    async fn get_log(
        &mut self,
        _req: get_log::Request,
    ) -> Result<impl Stream<Item = get_log::Response>, Self::Error> {
        Ok(futures::stream::iter(core::iter::from_fn(|| {
            #[cfg(feature = "rrp-log")]
            {
                crate::task::logger::LOG_CHANNEL.try_receive().ok()
            }

            #[cfg(not(feature = "rrp-log"))]
            {
                None
            }
        })))
    }

    async fn get_now(&mut self, _req: get_now::Request) -> Result<get_now::Response, Self::Error> {
        Ok(embassy_time::Instant::now().as_millis())
    }
}

struct ServerTransport<'a, R: ReporterDriver> {
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
            .recv_rrp_data(buf)
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
