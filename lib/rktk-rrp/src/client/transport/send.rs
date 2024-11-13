use futures::io::AsyncWriteExt as _;
use futures::stream::StreamExt as _;
use serde::Serialize;

use crate::shared::Indicator;

use super::{ClientWriteTransport, SendError};

pub(crate) async fn send_request_header<T: ClientWriteTransport>(
    tp: &mut T,
    request_id: u8,
    endpoint_id: u8,
) -> Result<(), SendError> {
    tp.write_all(&[Indicator::Start as u8, request_id, endpoint_id])
        .await?;

    Ok(())
}

pub(crate) async fn send_request_body<T: ClientWriteTransport, S: Serialize>(
    tp: &mut T,
    data: &S,
    continues: bool,
) -> Result<(), SendError> {
    let mut buf = Vec::new();
    let serialized = postcard::to_slice(data, &mut buf)?;

    tp.write_all(&(serialized.len() as u32).to_le_bytes())
        .await
        .expect("Failed to send length");
    tp.write_all(serialized)
        .await
        .expect("Failed to send request body");

    if continues {
        tp.write_all(&[Indicator::StreamContinues as u8]).await?;
    } else {
        tp.write_all(&[Indicator::End as u8])
            .await
            .expect("Failed to send end indicator");
    }

    Ok(())
}

pub(crate) async fn send_stream_request<T: ClientWriteTransport, S: Serialize>(
    tp: &mut T,
    stream: impl futures::stream::Stream<Item = S>,
) -> Result<(), SendError> {
    let mut buf = Vec::new();
    let mut stream = core::pin::pin!(stream);

    while let Some(data) = stream.next().await {
        let serialized = postcard::to_slice(&data, &mut buf)?;

        send_request_body(tp, &serialized, true).await?;
    }

    tp.write_all(&[Indicator::End as u8]).await?;

    Ok(())
}
