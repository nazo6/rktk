//! rrp client. uses std.

use core::fmt::Display;

use crate::transport::{ReadTransport, TransportError, WriteTransport};

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
