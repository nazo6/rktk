use futures::io::AsyncReadExt as _;
use futures::{AsyncRead, Stream};
use serde::de::DeserializeOwned;

use crate::shared::Indicator;

use super::ReceiveError;

async fn recv_indicator<T: AsyncRead + Unpin>(tp: &mut T) -> Result<Indicator, ReceiveError> {
    let mut buf = [0u8; 1];
    tp.read_exact(&mut buf).await?;
    buf[0]
        .try_into()
        .map_err(|_| ReceiveError::FrameError("Invalid indicator".to_string()))
}

pub(crate) struct ResponseHeader {
    pub request_id: u8,
    pub status_code: u8,
}

pub(crate) async fn recv_response_header<T: AsyncRead + Unpin>(
    tp: &mut T,
) -> Result<ResponseHeader, ReceiveError> {
    if recv_indicator(tp).await? != Indicator::Start {
        return Err(ReceiveError::FrameError("Invalid start signal".to_string()));
    }

    let mut request_header = [0u8; 2];

    tp.read_exact(&mut request_header).await?;

    Ok(ResponseHeader {
        request_id: request_header[0],
        status_code: request_header[1],
    })
}

pub(crate) async fn recv_request_body<T: AsyncRead + Unpin, R: DeserializeOwned>(
    tp: &mut T,
) -> Result<(R, Indicator), ReceiveError> {
    let mut request_size = [0u8; 4];
    tp.read_exact(&mut request_size).await?;
    let request_size = u32::from_le_bytes(request_size);

    let mut buf = vec![0; request_size as usize];
    tp.read_exact(&mut buf[..]).await?;

    let deserialized = postcard::from_bytes::<R>(&buf[0..request_size as usize])?;

    let indicator = recv_indicator(tp).await?;

    Ok((deserialized, indicator))
}

// NOTE: What I actually want to do is unpin the stream here, but the stream returned by unfold cannot be unpinned, so the caller needs to use pin!.
// Perhaps it can be implemented by writing the stream by hand?
pub(crate) fn recv_stream_request<'a, 't: 'a, T: AsyncRead + Unpin, R: DeserializeOwned>(
    tp: &'t mut T,
) -> impl Stream<Item = R> + 'a {
    futures::stream::unfold((tp, false), move |(tp, mut stream_finished)| async move {
        if stream_finished {
            return None;
        }

        let Ok((res, indicator)) = recv_request_body(tp).await else {
            return None;
        };

        if indicator == Indicator::End {
            stream_finished = true;
        }

        Some((res, (tp, stream_finished)))
    })
}
