use core::fmt::Debug;
use ekv::ReadError;

#[derive(Debug)]
pub enum ConfigReadError<E: Debug> {
    ReadError(ReadError<E>),
    DecodeError,
}

impl<E: Debug> From<ReadError<E>> for ConfigReadError<E> {
    fn from(e: ReadError<E>) -> Self {
        ConfigReadError::ReadError(e)
    }
}

macro_rules! read_trait {
    ($($name:tt, $key:tt, $input:tt => $output:tt)*) => {
        paste! {

        #[allow(async_fn_in_trait)]
        pub trait ReadConfig {
            type Error;
            $(
            def_read_trait!([<read_ $name>], $key, $input => $output);
            )*
        }

        impl<F: Flash, M: RawMutex> ReadConfig for ReadTransaction<'_, F, M> {
            type Error = ConfigReadError<F::Error>;
            $(
            impl_read!([<read_ $name>], $key, $input => $output);
            )*
        }

        }
    };
}

macro_rules! def_read_trait {
    ($name:ident, $key:ident, none => $res:ty) => {
        async fn $name(&self) -> Result<$res, Self::Error>;
    };
    ($name:ident, $key:ident, idx => $res:ident) => {
        async fn $name(&self, idx: u32) -> Result<$res, Self::Error>;
    };
}

macro_rules! impl_read {
    ($name:ident, $key:ident, none => u8) => {
        async fn $name(&self) -> Result<u8, Self::Error> {
            let mut buf = [0u8; 1];
            self.read(&[CONFIG_VERSION, ConfigKey::$key as u8], &mut buf)
                .await?;
            Ok(buf[0])
        }
    };
    ($name:ident, $key:ident, none => u32) => {
        async fn $name(&self) -> Result<u32, Self::Error> {
            let mut buf = [0u8; 4];
            self.read(&[CONFIG_VERSION, ConfigKey::$key as u8], &mut buf)
                .await?;
            Ok(u32::from_le_bytes(buf))
        }
    };
    ($name:ident, $key:ident, none => $res:ident) => {
        async fn $name(&self) -> Result<$res, Self::Error> {
            use postcard::experimental::max_size::MaxSize;
            let mut buf = [0u8; $res::POSTCARD_MAX_SIZE];
            self.read(&[CONFIG_VERSION, ConfigKey::$key as u8], &mut buf)
                .await?;
            let res = postcard::from_bytes(&buf).map_err(|_| ConfigReadError::DecodeError)?;
            Ok(res)
        }
    };
    ($name:ident, $key:ident, idx => $res:ident) => {
        async fn $name(&self, idx: u32) -> Result<$res, Self::Error> {
            use postcard::experimental::max_size::MaxSize;
            let mut buf = [0u8; $res::POSTCARD_MAX_SIZE];
            let idx = idx.to_le_bytes();
            self.read(
                &[
                    CONFIG_VERSION,
                    ConfigKey::$key as u8,
                    idx[0],
                    idx[1],
                    idx[2],
                    idx[3],
                ],
                &mut buf,
            )
            .await?;
            let res = postcard::from_bytes(&buf).map_err(|_| ConfigReadError::DecodeError)?;
            Ok(res)
        }
    };
}

pub(super) use def_read_trait;
pub(super) use impl_read;
pub(super) use read_trait;
