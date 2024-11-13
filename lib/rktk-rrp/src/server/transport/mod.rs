use embedded_io_async::{Read, ReadExactError, Write};

use super::ServerHandlers;

pub mod recv;
pub mod send;

pub trait ServerReadTransport: Read {}
pub trait ServerWriteTransport: Write {}

#[derive(Debug, thiserror::Error)]
pub enum ServerTransportError<RT: ServerReadTransport, WT: ServerWriteTransport, H: ServerHandlers>
{
    #[error(transparent)]
    RecvError(#[from] ReceiveError<RT>),
    #[error(transparent)]
    SendError(#[from] SendError<WT>),
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(u8),
    #[error("handler error: {0}")]
    HandlerError(H::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError<RT: ServerReadTransport> {
    #[error("frame error: {0}")]
    FrameError(&'static str),
    #[error("io error: {0}")]
    ReadExact(ReadExactError<RT::Error>),
    #[error("deserialization error: {0}")]
    Deserialization(postcard::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError<WT: ServerWriteTransport> {
    #[error("serialization error")]
    Serialization(postcard::Error),
    #[error("io error")]
    Write(WT::Error),
}
