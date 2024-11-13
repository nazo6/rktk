use futures::io::AsyncWriteExt as _;
use futures::stream::StreamExt as _;
use serde::Serialize;

use crate::shared::Indicator;

use super::ClientWriteTransport;

pub(crate) async fn send_request_header<T: ClientWriteTransport>(
    tp: &mut T,
    request_id: u8,
    endpoint_id: u8,
) -> Result<(), &'static str> {
    tp.write_all(&[Indicator::Start as u8, request_id, endpoint_id])
        .await
        .map_err(|_| "Failed to send request header")?;

    Ok(())
}

pub(crate) async fn send_request_body<
    T: ClientWriteTransport,
    S: Serialize,
    const BUF_SIZE: usize,
>(
    tp: &mut T,
    data: &S,
    continues: bool,
) -> Result<(), &'static str> {
    let mut buf = [0u8; BUF_SIZE];
    let serialized =
        postcard::to_slice(data, &mut buf).map_err(|_| "Failed to serialize request")?;

    tp.write_all(&(serialized.len() as u32).to_le_bytes())
        .await
        .expect("Failed to send length");
    tp.write_all(serialized)
        .await
        .expect("Failed to send request body");

    if continues {
        tp.write_all(&[Indicator::StreamContinues as u8])
            .await
            .map_err(|_| "Failed to send continues indicator")?;
    } else {
        tp.write_all(&[Indicator::End as u8])
            .await
            .expect("Failed to send end indicator");
    }

    Ok(())
}

pub(crate) async fn send_stream_request<
    T: ClientWriteTransport,
    S: Serialize,
    const BUF_SIZE: usize,
>(
    tp: &mut T,
    stream: impl futures::stream::Stream<Item = Result<S, &'static str>>,
) -> Result<(), &'static str> {
    let mut buf = [0u8; BUF_SIZE];
    let mut stream = core::pin::pin!(stream);

    while let Some(data) = stream.next().await {
        let serialized =
            postcard::to_slice(&data, &mut buf).map_err(|_| "Failed to serialize request")?;

        if (send_request_body::<_, _, BUF_SIZE>(tp, &serialized, true).await).is_err() {
            return Err("Failed to send request body");
        }
    }

    tp.write_all(&[Indicator::End as u8])
        .await
        .map_err(|_| "Failed to send end indicator")?;

    Ok(())
}
