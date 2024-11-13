use embedded_io_async::{Read, ReadExactError, Write};

pub mod recv;
pub mod send;

pub trait ServerReadTransport: Read {}
pub trait ServerWriteTransport: Write {}

#[derive(Debug, thiserror::Error)]
pub enum ServerTransportError<RT: ServerReadTransport, WT: ServerWriteTransport> {
    #[error(transparent)]
    RecvError(#[from] ReceiveError<RT>),
    #[error(transparent)]
    SendError(#[from] SendError<WT>),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError<RT: ServerReadTransport> {
    #[error("frame error: {0}")]
    FrameError(&'static str),
    #[error("io error: {0}")]
    ReadExact(ReadExactError<RT::Error>),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError<WT: ServerWriteTransport> {
    #[error("serialization error")]
    Serialization(postcard::Error),
    #[error("io error")]
    Write(WT::Error),
}
