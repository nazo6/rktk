use postcard_rpc::endpoint;
use rktk_keymanager::keycode::KeyDef;

use crate::LayerMap;

endpoint!(
    GetInfoJson,
    (),
    heapless::String<4096>,
    "endpoints/get_keymap"
);

endpoint!(GetKeymap, (), LayerMap, "endpoints/get_keymap");

#[derive(postcard::experimental::schema::Schema)]
pub struct UpdateKey {
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub key: KeyDef,
}
endpoint!(UpdateKeymap, UpdateKey, (), "endpoints/update_keymap");
