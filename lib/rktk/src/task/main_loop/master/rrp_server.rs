use embedded_io_async::ErrorKind;
use futures::Stream;
use rktk_keymanager::state::State;
use rktk_rrp::{
    endpoints::get_layout_json,
    server::{
        transport::{ServerReadTransport, ServerWriteTransport},
        ServerHandlers,
    },
};

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
    ) -> Result<rktk_rrp::endpoints::KeyActionLoc, Self::Error> {
        todo!()
    }

    async fn get_layout_json(
        &mut self,
        _req: (),
    ) -> Result<impl Stream<Item = get_layout_json::StreamResponse>, Self::Error> {
        Ok(futures::stream::iter(
            KEYBOARD.layout.as_bytes().chunks(64).map(|chunk| {
                let mut vec = heapless::Vec::new();
                vec.extend_from_slice(chunk).unwrap();
                vec
            }),
        ))
    }

    async fn stream_test(
        &mut self,
        _req: impl Stream<Item = ()>,
    ) -> Result<impl Stream<Item = ()>, Self::Error> {
        Ok(futures::stream::once(async {}))
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

impl<R: ReporterDriver> embedded_io_async::ErrorType for ServerTransport<'_, R> {
    type Error = ErrorKind;
}
impl<R: ReporterDriver> embedded_io_async::Read for ServerTransport<'_, R> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.reporter
            .read_rrp_data(buf)
            .await
            .map_err(|_| ErrorKind::Other)
    }
}
impl<R: ReporterDriver> embedded_io_async::Write for ServerTransport<'_, R> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.reporter
            .send_rrp_data(buf)
            .await
            .map_err(|_| ErrorKind::Other)?;
        Ok(buf.len())
    }
}
impl<R: ReporterDriver> ServerReadTransport for ServerTransport<'_, R> {}
impl<R: ReporterDriver> ServerWriteTransport for ServerTransport<'_, R> {}
