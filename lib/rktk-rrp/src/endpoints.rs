use rktk_keymanager::keycode::KeyAction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(
    not(feature = "std"),
    derive(postcard::experimental::max_size::MaxSize)
)]
pub struct KeyActionLoc {
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub key: KeyAction,
}

pub mod get_keyboard_info {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[cfg_attr(feature = "specta", derive(specta::Type))]
    #[cfg_attr(
        not(feature = "std"),
        derive(postcard::experimental::max_size::MaxSize)
    )]
    pub struct KeyboardInfo {
        #[cfg(not(feature = "std"))]
        pub name: heapless::String<64>,
        #[cfg(feature = "std")]
        pub name: String,
        pub rows: u8,
        pub cols: u8,
        pub layers: u8,
    }

    pub type Request = ();
    pub type Response = KeyboardInfo;
}

pub mod get_layout_json {
    pub type Request = ();
    /// 64 bytes stream of JSON layout data
    #[cfg(not(feature = "std"))]
    pub type StreamResponse = heapless::Vec<u8, 64>;
    #[cfg(feature = "std")]
    pub type StreamResponse = Vec<u8>;
}

pub mod get_keymaps {
    pub type Request = ();
    pub type StreamResponse = super::KeyActionLoc;
}
pub mod set_keymaps {
    pub type StreamRequest = super::KeyActionLoc;
    pub type Response = ();
}
