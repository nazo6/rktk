//! rrp client. uses std.

use core::fmt::Display;

pub(crate) mod transport;

/// Client to make requests to the rrp server.
pub struct Client<RT: ReadTransport + Unpin, WT: WriteTransport + Unpin> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
}

impl<RT: ReadTransport + Unpin, WT: WriteTransport + Unpin> Client<RT, WT> {
    pub fn new(reader: RT, writer: WT) -> Self {
        Self { reader, writer }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError<RE: Display, WE: Display> {
    #[error(transparent)]
    Transport(#[from] TransportError<RE, WE>),
    #[error("failed: status={status}, message={message}")]
    Failed { status: u8, message: String },
}

macro_rules! generate_client {
    ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
        use crate::client::*;
        use crate::client::transport::{recv::*, send::*};
        use crate::shared::transport::*;

        impl<RT: ReadTransport + Unpin, WT: WriteTransport + Unpin> Client<RT, WT> {
            $(
                pub async fn $endpoint_name(&mut self, req: generate_client!(@gen_type $req_kind: $req_type)) -> Result<generate_client!(@gen_type $res_kind: $res_type), ClientError<RT::Error, WT::Error>> {
                    send_request_header(&mut self.writer, 0, $endpoint_id).await.map_err(TransportError::SendError)?;
                    generate_client!(@send_request $req_kind, self.writer, req).map_err(TransportError::SendError)?;

                    let res_header = recv_response_header(&mut self.reader).await.map_err(TransportError::RecvError)?;
                    if res_header.status_code != 0 {
                        let (message, _ ) = recv_request_body::<_, String>(&mut self.reader).await.map_err(TransportError::RecvError)?;
                        return Err(ClientError::Failed { status: res_header.status_code, message });
                    }

                    let res = generate_client!(@recv_response $res_kind, self.reader, $res_type);

                    Ok(res)
                }
)*
        }
    };

    (@send_request normal, $writer:expr, $req:expr) => {
        send_single_request_body(&mut $writer, &$req).await
    };
    (@send_request stream, $writer:expr, $req:expr) => {
        send_stream_request_body(&mut $writer, $req).await
    };

    (@recv_response normal, $reader:expr, $res_type:ty) => {
        recv_request_body::<_, $res_type>(&mut $reader).await.map_err(TransportError::RecvError)?.0
    };
    (@recv_response stream, $reader:expr, $res_type:ty) => {
        recv_stream_request(&mut $reader)
    };

    (@gen_type normal: $ty:ty) => { $ty };
    (@gen_type stream: $ty:ty) => { impl futures::stream::Stream<Item = $ty> + '_ };
}
pub(crate) use generate_client;

use crate::shared::transport::{ReadTransport, TransportError, WriteTransport};
