use core::fmt::Display;

use futures::StreamExt as _;
use serde::Serialize;

use crate::transport::Indicator;

use super::error::SendError;

#[cfg(feature = "client")]
use super::RequestHeader;
#[cfg(feature = "server")]
use super::ResponseHeader;

#[allow(async_fn_in_trait)]
pub trait WriteTransport<const BUF_SIZE: usize> {
    type Error: Display;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

    async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        let mut offset = 0;
        while offset < buf.len() {
            let written = self.write(&buf[offset..]).await?;
            offset += written;
        }
        Ok(())
    }
}

pub trait WriteTransportExt<const BUF_SIZE: usize>: WriteTransport<BUF_SIZE> {
    #[cfg(feature = "client")]
    // Step 1-3 (client): Send request header
    async fn send_request_header(
        &mut self,
        header: RequestHeader,
    ) -> Result<(), SendError<Self::Error>> {
        self.write_all(&[
            Indicator::Start as u8,
            header.request_id,
            header.endpoint_id,
        ])
        .await
        .map_err(SendError::Write)?;
        Ok(())
    }

    #[cfg(feature = "server")]
    // Step 1-3 (response): Send response header
    async fn send_response_header(
        &mut self,
        header: ResponseHeader,
    ) -> Result<(), SendError<Self::Error>> {
        self.write_all(&[Indicator::Start as u8, header.request_id, header.status])
            .await
            .map_err(SendError::Write)?;
        Ok(())
    }

    // Step 4-7 (normal): Send body
    async fn send_body_normal<S: Serialize>(
        &mut self,
        data: &S,
    ) -> Result<(), SendError<Self::Error>> {
        let mut buf = [0u8; BUF_SIZE];
        let serialized = postcard::to_slice(data, &mut buf).map_err(SendError::Serialization)?;
        self.write_all(&[Indicator::Continue as u8])
            .await
            .map_err(SendError::Write)?;
        self.send_response_body(serialized).await?;
        self.write_all(&[Indicator::End as u8])
            .await
            .map_err(SendError::Write)?;
        Ok(())
    }

    // Step 4-7 (stream): Send body stream
    async fn send_body_stream<S: Serialize>(
        &mut self,
        stream: impl futures::stream::Stream<Item = S>,
    ) -> Result<(), SendError<Self::Error>> {
        let mut stream = core::pin::pin!(stream);
        while let Some(data) = stream.next().await {
            let mut buf = [0u8; BUF_SIZE];
            let serialized =
                postcard::to_slice(&data, &mut buf).map_err(SendError::Serialization)?;
            self.write_all(&[Indicator::Continue as u8])
                .await
                .map_err(SendError::Write)?;
            self.send_response_body(serialized).await?;
        }

        self.write_all(&[Indicator::End as u8])
            .await
            .map_err(SendError::Write)?;

        Ok(())
    }

    // utils

    // Step 5,6: Send response body
    async fn send_response_body(&mut self, data: &[u8]) -> Result<(), SendError<Self::Error>> {
        self.write_all(&(data.len() as u32).to_le_bytes())
            .await
            .map_err(SendError::Write)?;
        self.write_all(data).await.map_err(SendError::Write)?;
        Ok(())
    }
}

impl<T, const BUF_SIZE: usize> WriteTransportExt<BUF_SIZE> for T where T: WriteTransport<BUF_SIZE> {}
