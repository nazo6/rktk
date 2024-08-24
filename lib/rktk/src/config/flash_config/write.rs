use core::fmt::Debug;
use ekv::WriteError;

#[derive(Debug)]
pub enum ConfigWriteError<E: Debug> {
    WriteError(WriteError<E>),
    EncodeError,
}

impl<E: Debug> From<WriteError<E>> for ConfigWriteError<E> {
    fn from(e: WriteError<E>) -> Self {
        ConfigWriteError::WriteError(e)
    }
}

macro_rules! write_trait {
    ($($name:tt, $key:tt, $input:tt => $output:tt)*) => {
        paste! {

        #[allow(async_fn_in_trait)]
        pub trait WriteConfig {
            type Error;
            $(
                def_write_trait!([<write_ $name>], $key, $input => $output);
                def_delete_trait!([<delete_ $name>], $key, $input => $output);
            )*
        }

        impl<F: Flash, M: RawMutex> WriteConfig for WriteTransaction<'_, F, M> {
            type Error = ConfigWriteError<F::Error>;
            $(
            impl_write!([<write_ $name>], $key, $input => $output);
            impl_delete!([<delete_ $name>], $key, $input => $output);
            )*
        }

        }

    };
}

macro_rules! def_write_trait {
    ($name:ident, $key:ident, none => $data:ty) => {
        async fn $name(&mut self, data: $data) -> Result<(), Self::Error>;
    };
    ($name:ident, $key:ident, idx => $res:ident) => {
        async fn $name(&mut self, idx: u32, data: &$res) -> Result<(), Self::Error>;
    };
}

macro_rules! def_delete_trait {
    ($name:ident, $key:ident, none => $data:ty) => {
        async fn $name(&mut self) -> Result<(), Self::Error>;
    };
    ($name:ident, $key:ident, idx => $res:ident) => {
        async fn $name(&mut self, idx: u32) -> Result<(), Self::Error>;
    };
}

macro_rules! impl_write {
    ($name:ident, $key:ident, none => u8) => {
        async fn $name(&mut self, data: u8) -> Result<(), Self::Error> {
            self.write(&[CONFIG_VERSION, ConfigKey::$key as u8], &[data])
                .await?;
            Ok(())
        }
    };
    ($name:ident, $key:ident, none => u32) => {
        async fn $name(&mut self, data: u32) -> Result<(), Self::Error> {
            self.write(
                &[CONFIG_VERSION, ConfigKey::$key as u8],
                &data.to_le_bytes(),
            )
            .await?;
            Ok(())
        }
    };
    ($name:ident, $key:ident, none => $res:ident) => {
        async fn $name(&mut self, data: $res) -> Result<(), Self::Error> {
            use postcard::experimental::max_size::MaxSize;
            let mut buf = [0u8; $res::POSTCARD_MAX_SIZE];
            postcard::to_slice(&data, &mut buf).map_err(|_| ConfigWriteError::EncodeError)?;
            self.write(&[CONFIG_VERSION, ConfigKey::$key as u8], &mut buf)
                .await?;
            Ok(())
        }
    };
    ($name:ident, $key:ident, idx => $res:ident) => {
        async fn $name(&mut self, idx: u32, data: &$res) -> Result<(), Self::Error> {
            use postcard::experimental::max_size::MaxSize;
            let mut buf = [0u8; $res::POSTCARD_MAX_SIZE];
            let idx = idx.to_le_bytes();
            postcard::to_slice(data, &mut buf).map_err(|_| ConfigWriteError::EncodeError)?;
            self.write(
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
            Ok(())
        }
    };
}

macro_rules! impl_delete {
    ($name:ident, $key:ident, none => $res:ident) => {
        async fn $name(&mut self) -> Result<(), Self::Error> {
            self.delete(&[CONFIG_VERSION, ConfigKey::$key as u8])
                .await?;
            Ok(())
        }
    };
    ($name:ident, $key:ident, idx => $res:ident) => {
        async fn $name(&mut self, idx: u32) -> Result<(), Self::Error> {
            let idx = idx.to_le_bytes();
            self.delete(&[
                CONFIG_VERSION,
                ConfigKey::$key as u8,
                idx[0],
                idx[1],
                idx[2],
                idx[3],
            ])
            .await?;
            Ok(())
        }
    };
}

pub(super) use def_delete_trait;
pub(super) use def_write_trait;
pub(super) use impl_delete;
pub(super) use impl_write;
pub(super) use write_trait;
