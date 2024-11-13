pub(crate) mod transport;
use transport::{ClientReadTransport, ClientWriteTransport};

/// Client to make requests to the rrp server.
pub struct Client<RT: ClientReadTransport, WT: ClientWriteTransport> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    FrameError,
    RecvError,
    SendError,
}

macro_rules! generate_client {
    ($($endpoint_id:tt: $endpoint_name:ident($req_kind:tt: $req_type:ty) -> $res_kind:tt: $res_type:ty;)*) => {
        use crate::client::transport::{recv::*, send::*, *};
        use crate::client::*;
        use crate::macro_space::gen_type;

        impl<RT: ClientReadTransport, WT: ClientWriteTransport> Client<RT, WT> {
            $(
                pub async fn $endpoint_name(&mut self, req: gen_type!($req_kind: $req_type)) -> Result<gen_type!($res_kind: $res_type), anyhow::Error> {
                    use anyhow::Context as _;
                    send_ep_name(&mut self.writer, stringify!($endpoint_name)).await?;
                    send_req($req_kind, &mut self.writer, stringify!($endpoint_name), req).await?;
                    Ok(get_res($res_kind, &mut self.reader, stringify!($endpoint_name)).await)
                }
            )*
        }
    };
}
pub(crate) use generate_client;
