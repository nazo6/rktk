use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _, DuplexStream};

use crate::shared::transport::{ReadTransport, WriteTransport};

pub struct TestReader(pub DuplexStream);
impl ReadTransport for TestReader {
    type Error = std::io::Error;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.0.read(buf).await
    }
}

pub struct TestWriter(pub DuplexStream);
impl WriteTransport for TestWriter {
    type Error = std::io::Error;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buf).await
    }
}
