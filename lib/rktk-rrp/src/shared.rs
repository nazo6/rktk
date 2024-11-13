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
