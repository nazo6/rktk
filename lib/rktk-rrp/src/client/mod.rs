//! rrp client. uses std.

pub(crate) mod transport;
use transport::{ClientReadTransport, ClientWriteTransport};

/// Client to make requests to the rrp server.
pub struct Client<RT: ClientReadTransport, WT: ClientWriteTransport> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
}

impl<RT: ClientReadTransport, WT: ClientWriteTransport> Client<RT, WT> {
    pub fn new(reader: RT, writer: WT) -> Self {
        Self { reader, writer }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error(transparent)]
    Transport(#[from] transport::ClientTransportError),
    #[error("failed: status={status}, message={message}")]
    Failed { status: u8, message: String },
}

macro_rules! generate_client {
    ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
        use crate::client::transport::{recv::*, send::*, *};
        use crate::client::*;

        impl<RT: ClientReadTransport, WT: ClientWriteTransport> Client<RT, WT> {
            $(
                pub async fn $endpoint_name(&mut self, req: generate_client!(@gen_type $req_kind: $req_type)) -> Result<generate_client!(@gen_type $res_kind: $res_type), ClientError> {
                    send_request_header(&mut self.writer, 0, $endpoint_id).await.map_err(|e| transport::ClientTransportError::SendError(e))?;
                    generate_client!(@send_request $req_kind, self.writer, req).map_err(|e| transport::ClientTransportError::SendError(e))?;

                    let res_header = recv_response_header(&mut self.reader).await.map_err(|e| transport::ClientTransportError::RecvError(e))?;
                    if res_header.status_code != 0 {
                        let (message, _ ) = recv_request_body::<_, String>(&mut self.reader, &mut [0u8; 1024]).await.map_err(|e| transport::ClientTransportError::RecvError(e))?;
                        return Err(ClientError::Failed { status: res_header.status_code, message });
                    }

                    let res = generate_client!(@recv_response $res_kind, self.reader, $res_type);

                    Ok(res)
                }
            )*
        }
    };

    (@send_request normal, $writer:expr, $req:expr) => {
        send_request_body(&mut $writer, &$req, false).await
    };
    (@send_request stream, $writer:expr, $req:expr) => {
        send_stream_request(&mut $writer, $req).await
    };

    (@recv_response normal, $reader:expr, $res_type:ty) => {
        recv_request_body::<_, $res_type>(&mut $reader, &mut [0u8; 1024]).await.map_err(|e| transport::ClientTransportError::RecvError(e))?.0
    };
    (@recv_response stream, $reader:expr, $res_type:ty) => {
        recv_stream_request(&mut $reader)
    };

    (@gen_type normal: $ty:ty) => { $ty };
    (@gen_type stream: $ty:ty) => { impl futures::stream::Stream<Item = $ty> + '_ };
}
pub(crate) use generate_client;
