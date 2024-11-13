use futures::stream::StreamExt as _;
use serde::Serialize;

use crate::shared::Indicator;

use super::{SendError, ServerWriteTransport};

pub(crate) async fn send_response_header<T: ServerWriteTransport>(
    tp: &mut T,
    request_id: u8,
    status_code: u8,
) -> Result<(), SendError<T>> {
    tp.write_all(&[Indicator::Start as u8, request_id, status_code])
        .await
        .map_err(SendError::Write)?;

    Ok(())
}

/// Send error; this is a convenience function to send a response with a status code of 1.
pub(crate) async fn send_error_response<T: ServerWriteTransport, const BUF_SIZE: usize>(
    tp: &mut T,
    request_id: u8,
    body: &str,
) -> Result<(), SendError<T>> {
    send_response_header(tp, request_id, 1).await?;
    send_response_body::<_, _, BUF_SIZE>(tp, &body, false).await
}

pub(crate) async fn send_response_body<
    T: ServerWriteTransport,
    S: Serialize,
    const BUF_SIZE: usize,
>(
    tp: &mut T,
    data: &S,
    continues: bool,
) -> Result<(), SendError<T>> {
    let mut buf = [0u8; BUF_SIZE];
    let serialized = postcard::to_slice(data, &mut buf).map_err(SendError::Serialization)?;

    tp.write_all(&(serialized.len() as u32).to_le_bytes())
        .await
        .map_err(SendError::Write)?;

    tp.write_all(serialized).await.map_err(SendError::Write)?;

    if continues {
        tp.write_all(&[Indicator::StreamContinues as u8])
            .await
            .map_err(SendError::Write)?;
    } else {
        tp.write_all(&[Indicator::End as u8])
            .await
            .map_err(SendError::Write)?;
    }

    Ok(())
}

pub(crate) async fn send_stream_response<
    T: ServerWriteTransport,
    S: Serialize,
    const BUF_SIZE: usize,
>(
    tp: &mut T,
    stream: impl futures::stream::Stream<Item = S>,
) -> Result<(), SendError<T>> {
    let mut buf = [0u8; BUF_SIZE];
    let mut stream = core::pin::pin!(stream);

    while let Some(data) = stream.next().await {
        let serialized = postcard::to_slice(&data, &mut buf).map_err(SendError::Serialization)?;

        send_response_body::<_, _, BUF_SIZE>(tp, &serialized, true).await?;
    }

    tp.write_all(&[Indicator::End as u8])
        .await
        .map_err(SendError::Write)?;

    Ok(())
}
