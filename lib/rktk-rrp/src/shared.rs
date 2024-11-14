#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Indicator {
    Start = 0x55,
    StreamContinues = 0xFF,
    End = 0x00,
}

impl TryFrom<u8> for Indicator {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::End),
            0x55 => Ok(Self::Start),
            0xFF => Ok(Self::StreamContinues),
            _ => Err("Invalid indicator"),
        }
    }
}

pub mod transport {
    use core::fmt::Display;

    // NOTE: First, I tried to use embedded_io_async::Read and embedded_io_async::Write instead of
    // defining ReadTransport and WriteTransport. However, it's error type doesn't require Display trait,
    // so I gave up using it.

    #[allow(async_fn_in_trait)]
    pub trait ReadTransport {
        type Error: Display;

        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;

        async fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
            let mut offset = 0;
            while offset < buf.len() {
                let read = self.read(&mut buf[offset..]).await?;
                offset += read;
            }
            Ok(())
        }
    }

    #[allow(async_fn_in_trait)]
    pub trait WriteTransport {
        type Error: Display;

        async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error>;

        async fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
            let mut offset = 0;
            while offset < buf.len() {
                let written = self.write(&buf[offset..]).await?;
                offset += written;
            }
            Ok(())
        }
    }

    pub use error::TransportError;

    pub mod error {
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
        }

        #[derive(Debug, thiserror::Error)]
        pub enum SendError<WE: Display> {
            #[error("serialization error")]
            Serialization(#[from] postcard::Error),
            #[error("io error")]
            Write(WE),
        }
    }
}
