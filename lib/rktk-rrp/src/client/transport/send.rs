use futures::stream::StreamExt as _;
use futures::{io::AsyncWriteExt as _, AsyncWrite};
use serde::Serialize;

use crate::shared::Indicator;

use super::SendError;

pub(crate) async fn send_request_header<T: AsyncWrite + Unpin>(
    tp: &mut T,
    request_id: u8,
    endpoint_id: u8,
) -> Result<(), SendError> {
    tp.write_all(&[Indicator::Start as u8, request_id, endpoint_id])
        .await?;

    Ok(())
}

async fn send_request_body<T: AsyncWrite + Unpin, S: Serialize>(
    tp: &mut T,
    data: &S,
) -> Result<(), SendError> {
    let mut buf = [0u8; 1024];
    let serialized = postcard::to_slice(data, &mut buf)?;

    tp.write_all(&(serialized.len() as u32).to_le_bytes())
        .await?;
    tp.write_all(serialized).await?;

    Ok(())
}

pub(crate) async fn send_single_request_body<T: AsyncWrite + Unpin, S: Serialize>(
    tp: &mut T,
    data: &S,
) -> Result<(), SendError> {
    send_request_body(tp, data).await?;

    tp.write_all(&[Indicator::End as u8]).await?;

    Ok(())
}

pub(crate) async fn send_stream_request_body<T: AsyncWrite + Unpin, S: Serialize>(
    tp: &mut T,
    stream: impl futures::stream::Stream<Item = S>,
) -> Result<(), SendError> {
    let mut stream = core::pin::pin!(stream);

    let mut first = true;
    while let Some(data) = stream.next().await {
        if !first {
            tp.write_all(&[Indicator::StreamContinues as u8]).await?;
        }
        first = false;

        send_request_body(tp, &data).await?;
    }

    tp.write_all(&[Indicator::End as u8]).await?;

    Ok(())
}
