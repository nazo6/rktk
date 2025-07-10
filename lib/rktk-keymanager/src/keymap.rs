//! Keymap related types

use macro_rules_attribute::apply;

use crate::{
    interface::state::input_event::EncoderDirection,
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
    pub layers: [Layer<ROW, COL, ENCODER_COUNT>; LAYER],
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
    pub const fn const_default() -> Self {
        Self {
            layers: [const { Layer::const_default() }; LAYER],
            tap_dance: [const { None }; TAP_DANCE_MAX_DEFINITIONS],
            combo: [const { None }; COMBO_KEY_MAX_DEFINITIONS],
        }
    }

    pub fn get_keyaction(&self, layer: usize, row: usize, col: usize) -> Option<&KeyAction> {
        if let Some(layer) = self.layers.get(layer)
            && let Some(row) = layer.keymap.get(row)
                && let Some(key) = row.get(col) {
                    return Some(key);
                }
        None
    }

    pub fn get_encoder_key(
        &self,
        mut layer_state: [bool; LAYER],
        encoder: usize,
        direction: EncoderDirection,
    ) -> Option<&KeyCode> {
        layer_state[0] = true;
        self.layers
            .iter()
            .zip(layer_state.iter())
            .rev()
            .filter_map(|(l, s)| if *s { Some(l) } else { None })
            .find_map(|l| match direction {
                EncoderDirection::Clockwise => l.encoder_keys[encoder].1.as_ref(),
                EncoderDirection::CounterClockwise => l.encoder_keys[encoder].0.as_ref(),
            })
    }
}

/// Layer definition
///
/// This structure holds information about layer. This contains keymap and arrowmouse flag.
#[apply(common_derive)]
pub struct Layer<const ROW: usize, const COL: usize, const ENCODER_COUNT: usize> {
    // NOTE: This is workaround for issue that serde_as cannot be used with cfg-attr.
    // ref: https://github.com/jonasbb/serde_with/issues/355
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[[serde_with::Same; COL]; ROW]>")
    )]
    pub keymap: LayerKeymap<ROW, COL>,
    /// Keycode assigned to each encoder.
    ///
    /// Left of tuple is for counter clockwise, right of tuple is for clockwise.
    /// None has special meaning that it is not assigned and inherits keycode from previous layer.
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<[serde_with::Same; ENCODER_COUNT]>")
    )]
    pub encoder_keys: [(Option<KeyCode>, Option<KeyCode>); ENCODER_COUNT],
    pub arrow_mouse: bool,
}

impl<const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
    Layer<ROW, COL, ENCODER_COUNT>
{
    pub const fn const_default() -> Self {
        Self {
            keymap: [[KeyAction::const_default(); COL]; ROW],
            encoder_keys: [(None, None); ENCODER_COUNT],
            arrow_mouse: false,
        }
    }
}

impl<const ROW: usize, const COL: usize, const ENCODER_COUNT: usize> Default
    for Layer<ROW, COL, ENCODER_COUNT>
{
    fn default() -> Self {
        Self::const_default()
    }
}

/// Keymap of single layer
///
/// Type that represents keymap for each layer.
pub type LayerKeymap<const ROW: usize, const COL: usize> = [[KeyAction; COL]; ROW];

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
