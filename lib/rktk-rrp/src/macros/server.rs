macro_rules! gen_ep_sig {
    ($ep:ident, normal: $ty_req:ty, normal: $ty_res:ty) => {
        async fn $ep(&mut self, req: $ty_req) -> Result<$ty_res, Self::Error>;
    };
    ($ep:ident, normal: $ty_req:ty, stream: $ty_res:ty) => {
        async fn $ep(&mut self, req: $ty_req) -> Result<impl Stream<Item = $ty_res>, Self::Error>;
    };
    ($ep:ident, stream: $ty_req:ty, normal: $ty_res:ty) => {
        async fn $ep(
            &mut self,
            req: impl Stream<Item = Result<$ty_req, ReceiveError<RE>>>,
        ) -> Result<$ty_res, Self::Error>;
    };
    ($ep:ident, stream: $ty_req:ty, stream: $ty_res:ty) => {
        async fn $ep(
            &mut self,
            req: impl Stream<Item = Result<$ty_req, ReceiveError<RE>>>,
        ) -> Result<impl Stream<Item = $ty_res>, Self::Error>;
    };
}
pub(crate) use gen_ep_sig;

macro_rules! recv_request_body {
    (normal, $reader:expr) => {
        $reader.recv_body_normal().await?
    };
    (stream, $reader:expr) => {
        $reader.recv_body_stream().await
    };
}
pub(crate) use recv_request_body;

macro_rules! send_response_body {
    (normal, $writer:expr, $data:expr) => {
        $writer.send_body_normal(&$data).await
    };
    (stream, $writer:expr, $data:expr) => {
        $writer.send_body_stream($data).await
    };
}
pub(crate) use send_response_body;

macro_rules! generate_server_handlers {
    ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
        use core::fmt::Display;

        use $crate::macros::server::*;
        use $crate::server::*;
        use $crate::transport::*;
        use $crate::transport::error::*;
        use $crate::transport::read::ReadTransportExt as _;
        use $crate::transport::write::WriteTransportExt as _;

        use futures::Stream;

        #[allow(async_fn_in_trait)]
        pub trait ServerHandlers<RE: Display, WE: Display> {
            type Error: Display;
            $(
                gen_ep_sig!($endpoint_name, $req_kind: $req_type, $res_kind: $res_type);
            )*
        }

        impl<
                RT: ReadTransport<BUF_SIZE>,
                WT: WriteTransport<BUF_SIZE>,
                H: ServerHandlers<RT::Error, WT::Error>,
                const BUF_SIZE: usize,
            > Server<RT, WT, H, BUF_SIZE>
        {
            pub(crate) async fn handle(&mut self, header: RequestHeader) -> Result<(), TransportError<RT::Error, WT::Error>> {
                match header.endpoint_id {
                    $(
                        $endpoint_id => {
                            let req = recv_request_body!($req_kind, self.reader);

                            let Ok(res) = self.handlers.$endpoint_name(req).await else {
                                return Ok(());
                            };

                            self.writer.send_response_header(ResponseHeader {
                                request_id: header.request_id,
                                status: 0,
                            }).await?;

                            send_response_body!($res_kind, self.writer, res)?;
                        }
                    )*
                    _ => {
                        // send_error_response::<_, BUF_SIZE>(&mut self.writer, header.request_id, "Invalid enpoint").await?;
                    }
                }

                Ok(())
            }
        }
    };
}
pub(crate) use generate_server_handlers;
