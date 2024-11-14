macro_rules! req_type {
    (normal: $ty:ty) => { $ty };
    (stream: $ty:ty) => { impl futures::stream::Stream<Item = $ty> + '_ };
}
pub(crate) use req_type;

macro_rules! res_type {
    (normal: $ty:ty) => { $ty };
    (stream: $ty:ty) => { impl futures::stream::Stream<Item = Result<$ty, ReceiveError<RT::Error>>> + '_ };
}
pub(crate) use res_type;

macro_rules! send_request {
    (normal, $writer:expr, $req:expr) => {
        $writer.send_body_normal(&$req).await
    };
    (stream, $writer:expr, $req:expr) => {
        $writer.send_body_stream($req).await
    };
}
pub(crate) use send_request;

macro_rules! recv_response {
    (normal, $reader:expr) => {
        $reader
            .recv_body_normal()
            .await
            .map_err(TransportError::RecvError)?
    };
    (stream, $reader:expr) => {
        $reader.recv_body_stream().await
    };
}
pub(crate) use recv_response;

macro_rules! generate_client {
    ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
        use $crate::client::*;
        use $crate::macros::client::*;
        use $crate::transport::*;
        use $crate::transport::error::*;
        use $crate::transport::read::ReadTransportExt as _;
        use $crate::transport::write::WriteTransportExt as _;

        impl<
                RT: ReadTransport<BUF_SIZE> + Unpin,
                WT: WriteTransport<BUF_SIZE> + Unpin,
                const BUF_SIZE: usize,
            > Client<RT, WT, BUF_SIZE>
        {
            $(
                pub async fn $endpoint_name(&mut self, req: req_type!($req_kind: $req_type)) -> Result<res_type!($res_kind: $res_type), ClientError<RT::Error, WT::Error>> {
                    self.writer.send_request_header(RequestHeader {
                        request_id: 0,
                        endpoint_id: $endpoint_id,
                    }).await.map_err(TransportError::SendError)?;
                    send_request!($req_kind, self.writer, req).map_err(TransportError::SendError)?;

                    let res_header = self.reader.recv_response_header().await.map_err(TransportError::RecvError)?;
                    if res_header.status != 0 {
                        let message = self.reader.recv_body_normal().await.map_err(TransportError::RecvError)?;

                        return Err(ClientError::Failed { status: res_header.status, message });
                    }

                    let res = recv_response!($res_kind, self.reader);

                    Ok(res)
                }
 )*
        }
    };
}
pub(crate) use generate_client;
