use futures::stream::StreamExt as _;
use serde::Serialize;

use crate::shared::{
    transport::{error::SendError, WriteTransport},
    Indicator,
};

pub(crate) async fn send_request_header<T: WriteTransport + Unpin>(
    tp: &mut T,
    request_id: u8,
    endpoint_id: u8,
) -> Result<(), SendError<T::Error>> {
    tp.write_all(&[Indicator::Start as u8, request_id, endpoint_id])
        .await
        .map_err(SendError::Write)?;

    Ok(())
}

async fn send_request_body<T: WriteTransport + Unpin, S: Serialize>(
    tp: &mut T,
    data: &S,
) -> Result<(), SendError<T::Error>> {
    let serialized = postcard::to_stdvec(data)?;

    tp.write_all(&(serialized.len() as u32).to_le_bytes())
        .await
        .map_err(SendError::Write)?;

    tp.write_all(&serialized).await.map_err(SendError::Write)?;

    Ok(())
}

pub(crate) async fn send_single_request_body<T: WriteTransport + Unpin, S: Serialize>(
    tp: &mut T,
    data: &S,
) -> Result<(), SendError<T::Error>> {
    send_request_body(tp, data).await?;

    tp.write_all(&[Indicator::End as u8])
        .await
        .map_err(SendError::Write)?;

    Ok(())
}

pub(crate) async fn send_stream_request_body<T: WriteTransport + Unpin, S: Serialize>(
    tp: &mut T,
    stream: impl futures::stream::Stream<Item = S>,
) -> Result<(), SendError<T::Error>> {
    let mut stream = core::pin::pin!(stream);

    let mut first = true;
    while let Some(data) = stream.next().await {
        if !first {
            tp.write_all(&[Indicator::StreamContinues as u8])
                .await
                .map_err(SendError::Write)?;
        }
        first = false;

        send_request_body(tp, &data).await?;
    }

    tp.write_all(&[Indicator::End as u8])
        .await
        .map_err(SendError::Write)?;

    Ok(())
}
