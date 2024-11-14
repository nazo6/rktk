use embedded_io_async::{Error, Read, ReadExactError, Write};

pub mod recv;
pub mod send;

pub trait ServerReadTransport: Read {}
pub trait ServerWriteTransport: Write {}

#[derive(Debug, thiserror::Error)]
pub enum ServerTransportError<RE: Error, WE: Error> {
    #[error(transparent)]
    RecvError(#[from] ReceiveError<RE>),
    #[error(transparent)]
    SendError(#[from] SendError<WE>),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError<RE: Error> {
    #[error("frame error: {0}")]
    FrameError(&'static str),
    #[error("io error: {0}")]
    ReadExact(ReadExactError<RE>),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError<WE: Error> {
    #[error("serialization error")]
    Serialization(postcard::Error),
    #[error("io error")]
    Write(WE),
}
