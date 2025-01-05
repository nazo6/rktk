//! Keymap related types

use macro_rules_attribute::apply;

use crate::{
    interface::state::event::EncoderDirection,
    keycode::{KeyAction, KeyCode},
    macros::common_derive,
};

/// Root keymap type
///
/// This structure holds all information about keymap.
#[derive(Clone)]
pub struct Keymap<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    pub layers: [Layer<ROW, COL>; LAYER],
    pub encoder_keys: [(KeyCode, KeyCode); ENCODER_COUNT],
    pub tap_dance: TapDanceDefinitions<TAP_DANCE_MAX_DEFINITIONS, TAP_DANCE_MAX_REPEATS>,
    pub combo: ComboDefinitions<COMBO_KEY_MAX_DEFINITIONS, COMBO_KEY_MAX_SOURCES>,
}

impl<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >
    Keymap<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >
{
    pub fn get_keyaction(&self, layer: usize, row: usize, col: usize) -> Option<&KeyAction> {
        if let Some(layer) = self.layers.get(layer) {
            if let Some(row) = layer.map.get(row) {
                if let Some(key) = row.get(col) {
                    return Some(key);
                }
            }
        }
        None
    }

    pub fn get_encoder_key(&self, encoder: usize, direction: EncoderDirection) -> Option<&KeyCode> {
        if let Some(key) = self.encoder_keys.get(encoder) {
            match direction {
                EncoderDirection::Clockwise => Some(&key.1),
                EncoderDirection::CounterClockwise => Some(&key.0),
            }
        } else {
            None
        }
    }
}

/// Layer definition
///
/// This structure holds information about layer. This contains keymap and arrowmouse flag.
#[apply(common_derive)]
pub struct Layer<const ROW: usize, const COL: usize> {
    // NOTE: This is workaround for issue that serde_as cannot be used with cfg-attr.
    // ref: https://github.com/jonasbb/serde_with/issues/355
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[[serde_with::Same; COL]; ROW]>")
    )]
    pub map: LayerMap<ROW, COL>,
    pub arrowmouse: bool,
}

impl<const ROW: usize, const COL: usize> Default for Layer<ROW, COL> {
    fn default() -> Self {
        Self {
            map: [[KeyAction::default(); COL]; ROW],
            arrowmouse: false,
        }
    }
}

/// Type alias for layer map
///
/// Type that represents keymap for each layer.
pub type LayerMap<const ROW: usize, const COL: usize> = [[KeyAction; COL]; ROW];

/// Tap dance definition
#[apply(common_derive)]
pub struct TapDanceDefinition<const MAX_REPEATS: usize> {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[serde_with::Same; MAX_REPEATS]>")
    )]
    pub tap: [Option<KeyCode>; MAX_REPEATS],

    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[serde_with::Same; MAX_REPEATS]>")
    )]
    pub hold: [Option<KeyCode>; MAX_REPEATS],
}

pub type TapDanceDefinitions<const MAX_DEFINITIONS: usize, const MAX_REPEATS: usize> =
    [Option<TapDanceDefinition<MAX_REPEATS>>; MAX_DEFINITIONS];

#[apply(common_derive)]
pub struct ComboDefinition<const MAX_SOURCES: usize> {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[serde_with::Same; MAX_SOURCES]>")
    )]
    pub src: [Option<KeyCode>; MAX_SOURCES],
    pub dst: KeyCode,
}
pub type ComboDefinitions<const MAX_DEFINITIONS: usize, const MAX_SOURCES: usize> =
    [Option<ComboDefinition<MAX_SOURCES>>; MAX_DEFINITIONS];
