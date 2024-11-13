pub use crate::macro_space::server::*;
use transport::{recv::*, ServerReadTransport, ServerTransportError, ServerWriteTransport};

pub mod transport;

pub struct Server<
    RT: ServerReadTransport,
    WT: ServerWriteTransport,
    H: ServerHandlers,
    const BUF_SIZE: usize,
> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
    pub(crate) handlers: H,
}

impl<
        RT: ServerReadTransport,
        WT: ServerWriteTransport,
        H: ServerHandlers,
        const BUF_SIZE: usize,
    > Server<RT, WT, H, BUF_SIZE>
{
    pub fn new(reader: RT, writer: WT, handlers: H) -> Self {
        Self {
            reader,
            writer,
            handlers,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let _ = self.process_request().await;
        }
    }

    async fn process_request(&mut self) -> Result<(), ServerTransportError<RT, WT, H>> {
        let endpoint_id = recv_request_header(&mut self.reader)
            .await
            .map_err(ServerTransportError::RecvError)?;

        self.handle(endpoint_id).await?;

        Ok(())
    }
}

macro_rules! generate_server_handlers {
    ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
        use crate::macro_space::gen_type;
        use crate::server::*;
        use crate::server::transport::{recv::*, send::*, *};
        use core::fmt::Display;

        #[allow(async_fn_in_trait)]
        pub trait ServerHandlers {
            type Error: Display;
            $(
                async fn $endpoint_name(&mut self, req: gen_type!($req_kind: $req_type)) -> Result<gen_type!($res_kind: $res_type), Self::Error>;
            )*
        }


        impl<
                RT: ServerReadTransport,
                WT: ServerWriteTransport,
                H: ServerHandlers,
                const BUF_SIZE: usize,
            > Server<RT, WT, H, BUF_SIZE>
        {
            pub(crate) async fn handle(&mut self, header: RequestHeader) -> Result<(), ServerTransportError<RT, WT, H>> {
                match header.endpoint_id {
                    $(
                        $endpoint_id => {
                            let mut buf = [0u8; BUF_SIZE];
                            let req = generate_server_handlers!(@recv_req $req_kind, $endpoint_name, &mut self.reader, self.handlers, &mut buf);
                            let res = self.handlers.$endpoint_name(req).await.map_err(ServerTransportError::HandlerError)?;
                            send_response_header(&mut self.writer, header.request_id).await?;
                            generate_server_handlers!(@send_res $res_kind, res, &mut self.writer);

                        }
                    )*
                    _ => {
                        return Err(ServerTransportError::InvalidEndpoint(header.endpoint_id));
                    }
                }

                Ok(())
            }
        }
    };

    (@recv_req normal, $endpoint_name:ident, $tp:expr, $handlers:expr, $buf:expr) => {{
        recv_request_body($tp, $buf).await.map(|(req, _)| req)?
    }};
    (@recv_req stream, $endpoint_name:ident, $tp:expr, $handlers:expr, $buf:expr) => {{
        recv_stream_request($tp, $buf)
    }};

    (@send_res normal, $res:expr, $tp:expr) => {{
        send_response_body::<_, _, BUF_SIZE>($tp, &$res, false).await?
    }};
    (@send_res stream, $res:expr, $tp:expr) => {{
        send_stream_response::<_, _, BUF_SIZE>($tp, $res).await?
    }};
}
pub(crate) use generate_server_handlers;