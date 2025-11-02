use macro_rules_attribute::apply;

use super::common_derive;

#[apply(common_derive)]
pub struct KeymapInfo {
    pub layer_count: u8,
    pub max_tap_dance_key_count: u8,
    pub max_tap_dance_repeat_count: u8,
    pub oneshot_state_size: u8,
}

pub mod config {
    use crate::macros::common_derive;
    use macro_rules_attribute::apply;

    /// Configuration to initialize the keyboard state.
    #[apply(common_derive)]
    pub struct StateConfig {
        pub mouse: MouseConfig,
        pub key_resolver: KeyResolverConfig,
    }

    #[apply(common_derive)]
    pub struct MouseConfig {
        pub auto_mouse_layer: u8,
        pub auto_mouse_duration: u32,
        pub auto_mouse_threshold: u8,
        pub scroll_divider_x: i8,
        pub scroll_divider_y: i8,
    }

    #[apply(common_derive)]
    pub struct KeyResolverConfig {
        pub tap_hold: TapHoldConfig,
        pub tap_dance: TapDanceConfig,
        pub combo: ComboConfig,
    }

    #[apply(common_derive)]
    pub struct TapHoldConfig {
        pub threshold: u32,
        pub hold_on_other_key: bool,
    }

    #[apply(common_derive)]
    pub struct TapDanceConfig {
        pub threshold: u32,
    }

    #[apply(common_derive)]
    pub struct ComboConfig {
        pub threshold: u32,
    }
}

pub mod input_event {
    use crate::macros::common_derive;

    use macro_rules_attribute::apply;

    /// Represents a key event.
    ///
    /// Used generically to indicate that the state of a physical key has changed
    #[apply(common_derive)]
    pub struct KeyChangeEvent {
        pub col: u8,
        pub row: u8,
        pub pressed: bool,
    }

    /// Represents the direction of an encoder
    #[derive(Copy)]
    #[apply(common_derive)]
    pub enum EncoderDirection {
        Clockwise,
        CounterClockwise,
    }

    #[apply(common_derive)]
    pub enum InputEvent {
        Key(KeyChangeEvent),
        Mouse((i8, i8)),
        Encoder((u8, EncoderDirection)),
        None,
    }
}

pub mod output_event {
    use crate::{keycode::prelude::*, macros::common_derive};

    use macro_rules_attribute::apply;

    #[derive(Copy)]
    #[apply(common_derive)]
    pub enum EventType {
        Pressed,
        Pressing,
        Released,
    }

    #[derive(Copy)]
    #[apply(common_derive)]
    pub enum OutputEvent {
        KeyCode((KeyCode, EventType)),
        MouseMove((i8, i8)),
        MouseScroll((i8, i8)),
    }
}
