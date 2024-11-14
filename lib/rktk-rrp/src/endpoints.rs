use macro_rules_attribute::{apply, attribute_alias};
pub use rktk_keymanager;
use rktk_keymanager::keycode::KeyAction;

#[cfg(test)]
mod test_endpoints {
    pub mod test_normal_normal {
        pub type Request = String;
        pub type Response = String;
    }
    pub mod test_stream_normal {
        pub type Request = String;
        pub type Response = Vec<String>;
    }
    pub mod test_normal_stream {
        pub type Request = Vec<String>;
        pub type Response = String;
    }
    pub mod test_stream_stream {
        pub type Request = String;
        pub type Response = String;
    }
}
#[cfg(test)]
pub use test_endpoints::*;

attribute_alias! {
    #[apply(common_derive)] =
        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "specta", derive(specta::Type))]
        #[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
        #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
        #[cfg_attr(
            not(feature = "std"),
            derive(postcard::experimental::max_size::MaxSize)
        )]
    ;
}

#[apply(common_derive)]
pub struct KeyActionLoc {
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub key: KeyAction,
}

pub mod get_keyboard_info {
    use macro_rules_attribute::apply;
    use rktk_keymanager::state::config::KeymapInfo;

    #[apply(super::common_derive)]
    pub struct KeyboardInfo {
        #[cfg(not(feature = "std"))]
        pub name: heapless::String<64>,
        #[cfg(feature = "std")]
        pub name: String,
        pub rows: u8,
        pub cols: u8,
        pub keymap: KeymapInfo,
    }

    pub type Request = ();
    pub type Response = KeyboardInfo;
}

pub mod get_layout_json {
    pub type Request = ();
    /// 64 bytes stream of JSON layout data
    #[cfg(not(feature = "std"))]
    pub type Response = heapless::Vec<u8, 64>;
    #[cfg(feature = "std")]
    pub type Response = Vec<u8>;
}

pub mod get_keymaps {
    pub type Request = ();
    pub type Response = super::KeyActionLoc;
}
pub mod set_keymaps {
    pub type Request = super::KeyActionLoc;
    pub type Response = ();
}

pub mod get_now {
    pub type Request = ();
    pub type Response = u64;
}

pub mod get_keymap_config {
    pub type Request = ();
    pub type Response = rktk_keymanager::state::config::StateConfig;
}
pub mod set_keymap_config {
    pub type Request = rktk_keymanager::state::config::StateConfig;
    pub type Response = ();
}

pub mod get_log {
    use macro_rules_attribute::apply;

    #[serde_with::serde_as]
    #[apply(super::common_derive)]
    #[derive(Default)]
    pub enum LogLevel {
        Trace,
        Debug,
        #[default]
        Info,
        Warn,
        Error,
    }

    #[serde_with::serde_as]
    #[apply(super::common_derive)]
    pub enum LogChunk {
        Start {
            time: u64,
            level: LogLevel,
            line: Option<u32>,
        },
        Bytes {
            #[serde(with = "serde_with::As::<[serde_with::Same; 32]>")]
            bytes: [u8; 32],
            len: u8,
        },
        End,
    }

    pub type Request = ();
    pub type Response = LogChunk;
}
