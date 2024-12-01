//! Keymap related types

use crate::keycode::{KeyAction, KeyCode};

/// Root keymap type
///
/// This structure holds all information about keymap.
#[derive(Clone)]
pub struct Keymap<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
> {
    pub layers: [Layer<ROW, COL>; LAYER],
    pub encoder_keys: [(KeyCode, KeyCode); ENCODER_COUNT],
}

/// Layer definition
///
/// This structure holds information about layer. This contains keymap and arrowmouse flag.
#[cfg_attr(feature = "serde", serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "postcard",
    derive(postcard::experimental::max_size::MaxSize)
)]
#[derive(PartialEq, Eq, Clone, Debug)]
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
