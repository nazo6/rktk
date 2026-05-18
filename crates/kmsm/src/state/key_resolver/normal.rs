use crate::{
    interface::state::{input_event::KeyChangeEvent, output_event::EventType},
    keycode::{KeyAction, KeyCode},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ActiveKey {
    pub col: u8,
    pub row: u8,
    pub pressed_layer: u8,
}

/// State management for Normal and Normal2 action
pub struct NormalState<const MAX_PRESSED_KEYS: usize> {
    pressed: heapless::Vec<ActiveKey, MAX_PRESSED_KEYS>,
}

fn resolve_action<
    const LAYER: usize,
    const ROW: usize,
    const COL: usize,
    const ENCODER_COUNT: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
>(
    keymap: &crate::keymap::Keymap<
        LAYER,
        ROW,
        COL,
        ENCODER_COUNT,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >,
    layer: usize,
    row: usize,
    col: usize,
) -> Option<KeyAction> {
    let mut resolved_action = *keymap.get_keyaction(layer, row, col)?;
    if resolved_action == KeyAction::Inherit {
        for l in (0..layer).rev() {
            if let Some(a) = keymap.get_keyaction(l, row, col) {
                if *a != KeyAction::Inherit {
                    resolved_action = *a;
                    break;
                }
            }
        }
    }
    Some(resolved_action)
}

impl<const MAX_PRESSED_KEYS: usize> NormalState<MAX_PRESSED_KEYS> {
    pub fn new() -> Self {
        Self {
            pressed: heapless::Vec::new(),
        }
    }

    pub fn process_event<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >(
        &mut self,
        event: &KeyChangeEvent,
        key_action: KeyAction,
        highest_layer: usize,
        keymap: &crate::keymap::Keymap<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        out: &mut heapless::Vec<(KeyCode, EventType), 16>,
    ) {
        if event.pressed {
            let active_key = ActiveKey {
                col: event.col,
                row: event.row,
                pressed_layer: highest_layer as u8,
            };

            if !self.pressed.contains(&active_key) {
                let _ = self.pressed.push(active_key);

                match key_action {
                    KeyAction::Normal(k1) => {
                        let _ = out.push((k1, EventType::Pressed));
                    }
                    KeyAction::Normal2(k1, k2) => {
                        let _ = out.push((k1, EventType::Pressed));
                        let _ = out.push((k2, EventType::Pressed));
                    }
                    _ => {}
                }
            }
        } else {
            // release all keys that are pressed even if in other layers
            self.pressed.retain(|k| {
                if event.col == k.col && event.row == k.row {
                    if let Some(action) = resolve_action(keymap, k.pressed_layer as usize, event.row as usize, event.col as usize) {
                        match action {
                            KeyAction::Normal(k1) => {
                                let _ = out.push((k1, EventType::Released));
                            }
                            KeyAction::Normal2(k1, k2) => {
                                let _ = out.push((k1, EventType::Released));
                                let _ = out.push((k2, EventType::Released));
                            }
                            _ => {}
                        }
                    }
                    false
                } else {
                    true
                }
            });
        }
    }

    pub fn post_resolve<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
        const TAP_DANCE_MAX_DEFINITIONS: usize,
        const TAP_DANCE_MAX_REPEATS: usize,
        const COMBO_KEY_MAX_DEFINITIONS: usize,
        const COMBO_KEY_MAX_SOURCES: usize,
    >(
        &self,
        keymap: &crate::keymap::Keymap<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        out: &mut heapless::Vec<(KeyCode, EventType), 16>,
    ) {
        for k in self.pressed.iter() {
            if let Some(action) = resolve_action(keymap, k.pressed_layer as usize, k.row as usize, k.col as usize) {
                match action {
                    KeyAction::Normal(k1) => {
                        let _ = out.push((k1, EventType::Pressing));
                    }
                    KeyAction::Normal2(k1, k2) => {
                        let _ = out.push((k1, EventType::Pressing));
                        let _ = out.push((k2, EventType::Pressing));
                    }
                    _ => {}
                }
            }
        }
    }
}

