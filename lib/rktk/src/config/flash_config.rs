use ekv::{flash::Flash, ReadTransaction, WriteTransaction};
use embassy_sync::blocking_mutex::raw::RawMutex;
use paste::paste;
use rktk_keymanager::{keycode::KeyAction, state::config::StateConfig};

mod read;
mod write;

pub use read::*;
pub use write::*;

const CONFIG_VERSION: u8 = 0x00;

macro_rules! def_reader_writer {
    ($($id:expr, $name:tt, $input:tt => $output:ty )*) => {
        paste! {
            pub enum ConfigKey {
                $([<$name:camel>] = $id,)*
            }

            write_trait! {
                $($name, [<$name:camel>], $input => $output)*
            }
            read_trait! {
                $($name, [<$name:camel>], $input => $output)*
            }
        }
    };
}

def_reader_writer! {
    0, state_config, none => StateConfig
    1, keymap, idx => KeyAction
}
