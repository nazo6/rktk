use core::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum TransportError<RE: Display, WE: Display> {
    #[error(transparent)]
    RecvError(#[from] ReceiveError<RE>),
    #[error(transparent)]
    SendError(#[from] SendError<WE>),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError<RE: Display> {
    #[error("frame error: {0}")]
    FrameError(&'static str),
    #[error("io error: {0}")]
    Read(RE),
    #[error("deserialization error")]
    Deserialization(#[from] postcard::Error),
    #[error("buffer too small")]
    BufferTooSmall,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError<WE: Display> {
    #[error("serialization error")]
    Serialization(#[from] postcard::Error),
    #[error("io error")]
    Write(WE),
}
