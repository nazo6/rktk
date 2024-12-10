pub use crate::macros::server_generated::ServerHandlers;
use crate::transport::read::ReadTransportExt as _;
use crate::transport::*;

pub struct Server<
    RT: ReadTransport<BUF_SIZE>,
    WT: WriteTransport<BUF_SIZE>,
    H: ServerHandlers<RT::Error, WT::Error>,
    const BUF_SIZE: usize,
> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
    pub(crate) handlers: H,
}

impl<
        RT: ReadTransport<BUF_SIZE>,
        WT: WriteTransport<BUF_SIZE>,
        H: ServerHandlers<RT::Error, WT::Error>,
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

    async fn process_request(&mut self) -> Result<(), TransportError<RT::Error, WT::Error>> {
        let req_header = self.reader.recv_request_header().await?;

        log::error!("r{}", req_header.endpoint_id);

        self.handle(req_header).await?;

        Ok(())
    }
}
