pub mod recv;
pub mod send;

#[derive(Debug, thiserror::Error)]
pub enum ClientTransportError {
    #[error(transparent)]
    RecvError(#[from] ReceiveError),
    #[error(transparent)]
    SendError(#[from] SendError),
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(u8),
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError {
    #[error("frame error: {0}")]
    FrameError(String),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("deserialization error: {0}")]
    Deserialization(#[from] postcard::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("serialization error")]
    Serialization(#[from] postcard::Error),
    #[error("io error")]
    Io(#[from] std::io::Error),
}
