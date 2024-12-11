pub mod error;

pub(crate) mod read;
pub(crate) mod write;

pub use error::TransportError;
pub use read::ReadTransport;
pub use write::WriteTransport;

// NOTE: First, I tried to use embedded_io_async::Read and embedded_io_async::Write instead of
// defining ReadTransport and WriteTransport. However, it's error type doesn't require Display trait,
// so I gave up using it.

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Indicator {
    Start = 0x55,
    Continue = 0xFF,
    End = 0x00,
}

impl TryFrom<u8> for Indicator {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::End),
            0x55 => Ok(Self::Start),
            0xFF => Ok(Self::Continue),
            _ => Err("Invalid indicator"),
        }
    }
}

#[derive(Debug)]
pub struct RequestHeader {
    pub request_id: u8,
    pub endpoint_id: u8,
}

#[derive(Debug)]
pub struct ResponseHeader {
    pub request_id: u8,
    /// 0: OK, 1: Error
    pub status: u8,
}
