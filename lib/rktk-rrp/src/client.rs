//! rrp client. uses std.

use core::fmt::Display;

use crate::transport::{ReadTransport, TransportError, WriteTransport};

/// Client to make requests to the rrp server.
pub struct Client<
    RT: ReadTransport<BUF_SIZE> + Unpin,
    WT: WriteTransport<BUF_SIZE> + Unpin,
    const BUF_SIZE: usize,
> {
    pub(crate) reader: RT,
    pub(crate) writer: WT,
}

impl<
        RT: ReadTransport<BUF_SIZE> + Unpin,
        WT: WriteTransport<BUF_SIZE> + Unpin,
        const BUF_SIZE: usize,
    > Client<RT, WT, BUF_SIZE>
{
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
