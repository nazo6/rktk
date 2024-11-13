use futures::{channel::mpsc, future::join};
use test_server::handler::Handlers;
use tokio::io::duplex;

use crate::server::ServerHandlers;

#[tokio::test]
async fn test_normal() {
    // client -> server
    let output_channel = duplex(2048);
    // server -> client
    let input_channel = duplex(2048);
    join(
        async {
            // server

            let reader = test_server::ServerReader(output_channel.1);
            let writer = test_server::ServerWriter(input_channel.0);

            let mut server = crate::server::Server::<_, _, _, 1024>::new(reader, writer, Handlers);
            server.start().await;
        },
        async {
            // client
            let client = crate::client::Client::new(input_channel.1, output_channel.0);
        },
    )
    .await;
}

mod test_server {
    use embedded_io_async::{ErrorKind, ErrorType, Read, Write};
    use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _, DuplexStream};

    use crate::server::transport::{ServerReadTransport, ServerWriteTransport};

    pub struct ServerReader(pub DuplexStream);

    impl Read for ServerReader {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.0.read(buf).await.map_err(|_| ErrorKind::Other)
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
        use futures::Stream;

        use crate::{endpoints::get_layout_json, server::ServerHandlers};

        pub struct Handlers;

        impl ServerHandlers for Handlers {
            type Error = &'static str;

            async fn get_keyboard_info(
                &mut self,
                _req: (),
            ) -> Result<crate::endpoints::KeyActionLoc, Self::Error> {
                Err("Not implemented")
            }

            async fn get_layout_json(
                &mut self,
                _req: (),
            ) -> Result<impl Stream<Item = get_layout_json::StreamResponse>, Self::Error>
            {
                Ok(futures::stream::once(async { vec![0u8; 64] }))
            }

            async fn stream_test(
                &mut self,
                _req: impl Stream<Item = ()>,
            ) -> Result<impl Stream<Item = ()>, Self::Error> {
                Ok(futures::stream::once(async {}))
            }
        }
    }
}

mod test_client {
    use futures::AsyncRead as FuturesAsyncRead;
    use tokio::io::AsyncRead;
    use tokio::io::DuplexStream;

    pub struct ClientWriter(pub DuplexStream);
    impl FuturesAsyncRead for ClientWriter {
        fn poll_read(
            self: core::pin::Pin<&mut Self>,
            cx: &mut core::task::Context<'_>,
            buf: &mut [u8],
        ) -> core::task::Poll<std::io::Result<usize>> {
            AsyncRead::poll_read(core::pin::Pin::new(&mut self.get_mut().0), cx, buf)
        }
    }
}
