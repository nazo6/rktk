use rktk_keymanager::state::config::KeymapInfo;

use crate::endpoints::get_keyboard_info;

pub fn test_keyboard_info() -> get_keyboard_info::KeyboardInfo {
    get_keyboard_info::Response {
        name: "test".to_string(),
        rows: 5,
        cols: 10,
        keymap: KeymapInfo {
            layer_count: 5,
            max_tap_dance_key_count: 10,
            max_tap_dance_repeat_count: 10,
            oneshot_state_size: 10,
            max_resolved_key_count: 10,
        },
    }
}
