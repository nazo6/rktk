use rktk_keymanager::keycode::KeyDef;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(
    not(feature = "std-client"),
    derive(postcard::experimental::max_size::MaxSize)
)]
pub struct KeyDefLoc {
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub key: KeyDef,
}

pub mod get_keyboard_info {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "specta", derive(specta::Type))]
    #[cfg_attr(
        not(feature = "std-client"),
        derive(postcard::experimental::max_size::MaxSize)
    )]
    pub struct KeyboardInfo {
        #[cfg(not(feature = "std-client"))]
        pub name: heapless::String<64>,
        #[cfg(feature = "std-client")]
        pub name: String,
        pub rows: u8,
        pub cols: u8,
    }

    pub type Request = ();
    pub type Response = KeyboardInfo;
}

pub mod get_layout_json {
    pub type Request = ();
    /// 64 bytes stream of JSON layout data
    #[cfg(not(feature = "std-client"))]
    pub type StreamResponse = heapless::Vec<u8, 64>;
    #[cfg(feature = "std-client")]
    pub type StreamResponse = Vec<u8>;
}

pub mod get_keymaps {
    pub type Request = ();
    pub type StreamResponse = super::KeyDefLoc;
}
pub mod set_keymaps {
    pub type StreamRequest = super::KeyDefLoc;
    pub type Response = ();
}
