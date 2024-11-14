use core::fmt::Display;

use futures::Stream;
use serde::de::DeserializeOwned;

use super::{error::ReceiveError, Indicator};

#[cfg(feature = "server")]
use super::RequestHeader;
#[cfg(feature = "client")]
use super::ResponseHeader;

#[allow(async_fn_in_trait)]
pub trait ReadTransport<const BUF_SIZE: usize> {
    type Error: Display;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut offset = 0;
        while offset < buf.len() {
            let read = self.read(&mut buf[offset..]).await?;
            offset += read;
        }
        Ok(())
    }
}

pub trait ReadTransportExt<const BUF_SIZE: usize>: ReadTransport<BUF_SIZE> {
    #[cfg(feature = "server")]
    // Step 1-3 (request, server): Receive request header
    async fn recv_request_header(&mut self) -> Result<RequestHeader, ReceiveError<Self::Error>> {
        if self.recv_indicator().await? != Indicator::Start {
            return Err(ReceiveError::FrameError("Invalid start signal"));
        }
        let mut request_header = [0u8; 2];
        self.read_exact(&mut request_header)
            .await
            .map_err(ReceiveError::Read)?;
        Ok(RequestHeader {
            request_id: request_header[0],
            endpoint_id: request_header[1],
        })
    }

    #[cfg(feature = "client")]
    // Step 1-3 (response): Receive response header
    async fn recv_response_header(&mut self) -> Result<ResponseHeader, ReceiveError<Self::Error>> {
        if self.recv_indicator().await? != Indicator::Start {
            return Err(ReceiveError::FrameError("Invalid start signal"));
        }
        let mut response_header = [0u8; 2];
        self.read_exact(&mut response_header)
            .await
            .map_err(ReceiveError::Read)?;
        Ok(ResponseHeader {
            request_id: response_header[0],
            status: response_header[1],
        })
    }

    // Step 4-7 (normal): Receive body
    async fn recv_body_normal<D: DeserializeOwned>(
        &mut self,
    ) -> Result<D, ReceiveError<Self::Error>> {
        if self.recv_indicator().await? != Indicator::Continue {
            return Err(ReceiveError::FrameError("Invalid indicator"));
        }
        let deserialized = self.recv_body().await?;
        if self.recv_indicator().await? != Indicator::End {
            return Err(ReceiveError::FrameError("Invalid indicator"));
        }
        Ok(deserialized)
    }

    // Step 4-7 (stream): Receive body stream
    async fn recv_body_stream<D: DeserializeOwned>(
        &mut self,
    ) -> impl Stream<Item = Result<D, ReceiveError<Self::Error>>> {
        futures::stream::unfold(self, move |tp| async move {
            match tp.recv_indicator().await {
                Ok(Indicator::Start) => {
                    Some((Err(ReceiveError::FrameError("Invalid indicator")), tp))
                }
                Ok(Indicator::Continue) => {
                    let deserialized = tp.recv_body().await;
                    Some((deserialized, tp))
                }
                Ok(Indicator::End) => None,
                Err(e) => Some((Err(e), tp)),
            }
        })
    }

    // utils

    async fn recv_indicator(&mut self) -> Result<Indicator, ReceiveError<Self::Error>> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)
            .await
            .map_err(ReceiveError::Read)?;
        buf[0]
            .try_into()
            .map_err(|_| ReceiveError::FrameError("Invalid indicator"))
    }

    // Step 5,6
    async fn recv_body<R: DeserializeOwned>(&mut self) -> Result<R, ReceiveError<Self::Error>> {
        let mut request_size = [0u8; 4];
        self.read_exact(&mut request_size)
            .await
            .map_err(ReceiveError::Read)?;
        let request_size = u32::from_le_bytes(request_size);

        #[cfg(not(feature = "std"))]
        let mut buf = {
            if request_size as usize > BUF_SIZE {
                return Err(ReceiveError::BufferTooSmall);
            }
            [0u8; BUF_SIZE]
        };

        #[cfg(feature = "std")]
        let mut buf = vec![0; request_size as usize];

        self.read_exact(&mut buf[0..request_size as usize])
            .await
            .map_err(ReceiveError::Read)?;

        let deserialized = postcard::from_bytes::<R>(&buf[0..request_size as usize])?;

        Ok(deserialized)
    }
}

impl<T, const BUF_SIZE: usize> ReadTransportExt<BUF_SIZE> for T where T: ReadTransport<BUF_SIZE> {}
