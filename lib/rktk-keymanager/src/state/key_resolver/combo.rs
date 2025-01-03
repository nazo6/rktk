use crate::{
    config::ComboConfig,
    keycode::KeyCode,
    keymap::{ComboDefinition, ComboDefinitions},
    state::CONST_CONFIG,
    time::{Duration, Instant},
};

use super::EventType;

#[derive(Debug)]
enum ComboUnitState {
    None,
    Pending(
        (
            Instant,
            [bool; CONST_CONFIG.max_combo_combination_count as usize],
        ),
    ),
    Pressing([bool; CONST_CONFIG.max_combo_combination_count as usize]),
}

#[derive(Debug)]
struct ComboUnit {
    state: ComboUnitState,
    def: Option<ComboDefinition>,
}

#[derive(Debug)]
pub struct ComboState {
    state: [ComboUnit; CONST_CONFIG.max_combo_key_count as usize],
    config: ComboConfig,
}

impl ComboState {
    pub fn new(def: ComboDefinitions, config: ComboConfig) -> Self {
        Self {
            state: def.map(|def| ComboUnit {
                state: ComboUnitState::None,
                def,
            }),
            config,
        }
    }

    pub fn pre_resolve(&mut self, now: Instant, mut cb_direct: impl FnMut(EventType, KeyCode)) {
        for unit in self.state.iter_mut() {
            if let Some(def) = &mut unit.def {
                if let ComboUnitState::Pending((start, state)) = unit.state {
                    if now - start > Duration::from_millis(self.config.threshold) {
                        for (i, enabled) in state.iter().enumerate() {
                            if *enabled {
                                cb_direct(EventType::Pressed, def.src[i].unwrap());
                            }
                        }
                        unit.state = ComboUnitState::None;
                    }
                }
            }
        }
    }

    pub fn process_keycode(&mut self, event_type: &EventType, keycode: &mut KeyCode, now: Instant) {
        for unit in self.state.iter_mut() {
            if let Some(def) = &mut unit.def {
                for (i, key) in def.src.iter().enumerate() {
                    if let Some(key) = key {
                        if *keycode == *key {
                            #[cfg(test)]
                            dbg!(event_type, &unit.state);

                            match (event_type, &unit.state) {
                                (EventType::Pressed, ComboUnitState::None) => {
                                    unit.state = ComboUnitState::Pending((
                                        now,
                                        [false; CONST_CONFIG.max_combo_combination_count as usize],
                                    ));
                                    *keycode = KeyCode::None;
                                }
                                (
                                    EventType::Pressed | EventType::Pressing,
                                    ComboUnitState::Pending(mut state),
                                ) => {
                                    state.1[i] = true;
                                    if state
                                        .1
                                        .iter()
                                        .enumerate()
                                        .all(|(i, &b)| b || def.src[i].is_none())
                                    {
                                        unit.state = ComboUnitState::Pressing(state.1);
                                        *keycode = def.dst;
                                    } else {
                                        unit.state = ComboUnitState::Pending(state);
                                        *keycode = KeyCode::None;
                                    }
                                }
                                (
                                    EventType::Pressed | EventType::Pressing,
                                    ComboUnitState::Pressing(_),
                                ) => {
                                    *keycode = def.dst;
                                }
                                (EventType::Released, ComboUnitState::Pressing(mut state)) => {
                                    state[i] = false;
                                    if state.iter().all(|&b| !b) {
                                        unit.state = ComboUnitState::None;
                                        *keycode = KeyCode::None;
                                    } else {
                                        unit.state = ComboUnitState::Pressing(state);
                                        *keycode = def.dst;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}
