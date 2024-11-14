use embedded_io_async::{ErrorKind, ErrorType, Read, Write};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _, DuplexStream};

use crate::server::transport::{ServerReadTransport, ServerWriteTransport};

pub struct ServerReader(pub DuplexStream);

impl Read for ServerReader {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        loop {
            let size = self.0.read(buf).await.map_err(|_| ErrorKind::Other)?;
            if size != 0 {
                return Ok(size);
            }
        }
    }
}
impl ErrorType for ServerReader {
    type Error = ErrorKind;
}
impl ServerReadTransport for ServerReader {}

pub struct ServerWriter(pub DuplexStream);
impl Write for ServerWriter {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buf).await.map_err(|_| ErrorKind::Other)
    }
}
impl ErrorType for ServerWriter {
    type Error = ErrorKind;
}
impl ServerWriteTransport for ServerWriter {}

pub mod handler {
    use futures::StreamExt as _;
    use futures::{stream, Stream};

    use crate::server::ServerHandlers;

    pub struct Handlers;

    impl ServerHandlers for Handlers {
        type Error = &'static str;

        async fn test_normal_normal(&mut self, req: String) -> Result<String, Self::Error> {
            Ok(format!("req: {}", req))
        }

        async fn test_stream_normal(
            &mut self,
            req: impl Stream<Item = String>,
        ) -> Result<Vec<String>, Self::Error> {
            Ok(req
                .collect::<Vec<String>>()
                .await
                .into_iter()
                .map(|e| format!("req: {}", e))
                .collect())
        }

        async fn test_normal_stream(
            &mut self,
            req: String,
        ) -> Result<impl Stream<Item = String>, Self::Error> {
            Ok(stream::iter(vec![req.clone(), req.clone(), req.clone()]))
        }

        async fn test_stream_stream(
            &mut self,
            req: impl Stream<Item = String>,
        ) -> Result<impl Stream<Item = String>, Self::Error> {
            Ok(req)
        }
    }
}
