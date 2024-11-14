use futures::Stream;
use serde::de::DeserializeOwned;

use crate::shared::Indicator;

use super::{ReceiveError, ServerReadTransport};

async fn recv_indicator<T: ServerReadTransport>(
    tp: &mut T,
) -> Result<Indicator, ReceiveError<T::Error>> {
    let mut buf = [0u8; 1];
    tp.read_exact(&mut buf)
        .await
        .map_err(ReceiveError::ReadExact)?;
    buf[0]
        .try_into()
        .map_err(|_| ReceiveError::FrameError("Invalid indicator"))
}

#[derive(Debug)]
pub(crate) struct RequestHeader {
    pub request_id: u8,
    pub endpoint_id: u8,
}

pub(crate) async fn recv_request_header<T: ServerReadTransport>(
    tp: &mut T,
) -> Result<RequestHeader, ReceiveError<T::Error>> {
    if recv_indicator(tp).await? != Indicator::Start {
        return Err(ReceiveError::FrameError("Invalid start signal"));
    }

    let mut request_header = [0u8; 2];

    tp.read_exact(&mut request_header)
        .await
        .map_err(ReceiveError::ReadExact)?;

    Ok(RequestHeader {
        request_id: request_header[0],
        endpoint_id: request_header[1],
    })
}

pub(crate) async fn recv_request_body<T: ServerReadTransport, R: DeserializeOwned>(
    tp: &mut T,
    buf: &mut [u8],
) -> Result<(Result<R, postcard::Error>, Indicator), ReceiveError<T::Error>> {
    let mut request_size = [0u8; 4];
    tp.read_exact(&mut request_size)
        .await
        .map_err(ReceiveError::ReadExact)?;

    let request_size = u32::from_le_bytes(request_size);

    tp.read_exact(&mut buf[0..request_size as usize])
        .await
        .map_err(ReceiveError::ReadExact)?;

    let deserialized = postcard::from_bytes::<R>(&buf[0..request_size as usize]);

    let indicator = recv_indicator(tp).await?;

    Ok((deserialized, indicator))
}

pub(crate) fn recv_stream_request<'a, 't: 'a, T: ServerReadTransport, R: DeserializeOwned>(
    tp: &'t mut T,
    buf: &'a mut [u8],
) -> impl Stream<Item = R> + 'a {
    futures::stream::unfold(
        (tp, buf, false),
        move |(tp, buf, mut stream_finished)| async move {
            if stream_finished {
                return None;
            }

            let Ok((Ok(res), indicator)) = recv_request_body(tp, buf).await else {
                return None;
            };

            if indicator == Indicator::End {
                stream_finished = true;
            }

            dbg!(indicator);

            Some((res, (tp, buf, stream_finished)))
        },
    )
}
