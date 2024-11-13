use futures::io::AsyncReadExt as _;
use futures::Stream;
use serde::de::DeserializeOwned;

use crate::shared::Indicator;

use super::ClientReadTransport;

async fn recv_indicator<T: ClientReadTransport>(tp: &mut T) -> Result<Indicator, &'static str> {
    let mut buf = [0u8; 1];
    tp.read_exact(&mut buf)
        .await
        .map_err(|_| "Failed to receive indicator")?;
    buf[0].try_into().map_err(|_| "Invalid indicator")
}

pub(crate) struct ResponseHeader {
    pub request_id: u8,
}

pub(crate) async fn recv_response_header<T: ClientReadTransport>(
    tp: &mut T,
) -> Result<ResponseHeader, &'static str> {
    if recv_indicator(tp)
        .await
        .map_err(|_| "Failed to receive start indicator")?
        != Indicator::Start
    {
        return Err("Invalid start signal");
    }

    let mut request_header = [0u8; 1];

    let Ok(_) = tp.read_exact(&mut request_header).await else {
        return Err("Failed to response header");
    };

    Ok(ResponseHeader {
        request_id: request_header[0],
    })
}

pub(crate) async fn recv_request_body<T: ClientReadTransport, R: DeserializeOwned>(
    tp: &mut T,
    buf: &mut [u8],
) -> Result<(R, Indicator), &'static str> {
    let mut request_size = [0u8; 4];
    tp.read_exact(&mut request_size)
        .await
        .map_err(|_| "Failed to receive size")?;
    let request_size = u32::from_le_bytes(request_size);

    tp.read_exact(&mut buf[0..request_size as usize])
        .await
        .map_err(|_| "Failed to receive data")?;

    let deserialized = postcard::from_bytes::<R>(&buf[0..request_size as usize])
        .map_err(|_| "Failed to deserialize")?;

    let indicator = recv_indicator(tp)
        .await
        .map_err(|_| "Failed to receive indicator")?;

    Ok((deserialized, indicator))
}

pub(crate) fn recv_stream_request<'a, 't: 'a, T: ClientReadTransport, R: DeserializeOwned>(
    tp: &'t mut T,
    buf: &'a mut [u8],
) -> impl Stream<Item = Result<R, &'static str>> + 'a {
    futures::stream::unfold(
        (tp, buf, false),
        move |(tp, buf, mut stream_finished)| async move {
            if stream_finished {
                return None;
            }

            let Ok((res, indicator)) = recv_request_body(tp, buf).await else {
                return None;
            };

            if indicator == Indicator::End {
                stream_finished = true;
            }

            Some((Ok(res), (tp, buf, stream_finished)))
        },
    )
}
