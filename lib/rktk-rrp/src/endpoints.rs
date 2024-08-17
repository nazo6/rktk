use postcard::experimental::max_size::MaxSize;
use rktk_keymanager::keycode::KeyDef;
use serde::{Deserialize, Serialize};

#[derive(MaxSize, Serialize, Deserialize, Debug)]
pub struct KeyDefLoc {
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub key: KeyDef,
}

pub mod get_keyboard_info {
    use postcard::experimental::max_size::MaxSize;
    use serde::{Deserialize, Serialize};

    #[derive(MaxSize, Serialize, Deserialize, Debug)]
    pub struct KeyboardInfo {
        pub name: heapless::String<64>,
        pub rows: u8,
        pub cols: u8,
    }

    pub type Request = ();
    pub type Response = KeyboardInfo;
}

pub mod get_layout_json {
    pub type Request = ();
    pub type StreamResponse = heapless::String<64>;
}

pub mod get_keymaps {
    pub type Request = ();
    pub type StreamResponse = super::KeyDefLoc;
}
pub mod set_keymaps {
    pub type StreamRequest = super::KeyDefLoc;
    pub type Response = ();
}
