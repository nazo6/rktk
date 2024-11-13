use futures::{io::AsyncRead, AsyncWrite};

pub mod recv;
pub mod send;

pub trait ClientReadTransport: AsyncRead + Unpin {}
pub trait ClientWriteTransport: AsyncWrite + Unpin {}
