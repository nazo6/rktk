pub use crate::macros::server_generated::ServerHandlers;
use crate::transport::read::ReadTransportExt as _;
use crate::transport::*;

pub struct Server<RT: ReadTransport, WT: WriteTransport, H: ServerHandlers<RT::Error, WT::Error>> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
    pub(crate) handlers: H,
}

impl<RT: ReadTransport, WT: WriteTransport, H: ServerHandlers<RT::Error, WT::Error>>
    Server<RT, WT, H>
{
    pub fn new(reader: RT, writer: WT, handlers: H) -> Self {
        Self {
            reader,
            writer,
            handlers,
        }
    }

    pub async fn start<const BUF_SIZE: usize>(&mut self) {
        loop {
            let _ = self.process_request::<BUF_SIZE>().await;
        }
    }

    async fn process_request<const BUF_SIZE: usize>(
        &mut self,
    ) -> Result<(), TransportError<RT::Error, WT::Error>> {
        let req_header = self.reader.recv_request_header().await?;

        self.handle::<BUF_SIZE>(req_header).await?;

        Ok(())
    }
}
