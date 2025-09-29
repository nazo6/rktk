#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    interface::state::config::ComboConfig,
    keycode::KeyCode,
    keymap::{ComboDefinition, ComboDefinitions},
    time::{Duration, Instant},
};

use super::EventType;

#[derive(Debug)]
enum ComboUnitState<const MAX_SOURCES: usize> {
    None,
    Pending((Instant, [bool; MAX_SOURCES])),
    Pressing([bool; MAX_SOURCES]),
}

#[derive(Debug)]
struct ComboUnit<const MAX_SOURCES: usize> {
    state: ComboUnitState<MAX_SOURCES>,
    def: Option<ComboDefinition<MAX_SOURCES>>,
}

#[derive(Debug)]
pub struct ComboState<const MAX_DEFINITIONS: usize, const MAX_SOURCES: usize> {
    state: [ComboUnit<MAX_SOURCES>; MAX_DEFINITIONS],
    config: ComboConfig,
}

impl<const MAX_DEFINITIONS: usize, const MAX_SOURCES: usize>
    ComboState<MAX_DEFINITIONS, MAX_SOURCES>
{
    pub fn new(def: ComboDefinitions<MAX_DEFINITIONS, MAX_SOURCES>, config: ComboConfig) -> Self {
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
            if let Some(def) = &mut unit.def
                && let ComboUnitState::Pending((start, state)) = unit.state
                    && now - start > Duration::from_millis(self.config.threshold) {
                        for (i, enabled) in state.iter().enumerate() {
                            if *enabled {
                                cb_direct(EventType::Pressed, def.src[i].unwrap());
                            }
                        }
                        unit.state = ComboUnitState::None;
                    }
        }
    }

    pub fn process_keycode(&mut self, event_type: &EventType, keycode: &mut KeyCode, now: Instant) {
        for unit in self.state.iter_mut() {
            if let Some(def) = &mut unit.def {
                for (i, key) in def.src.iter().enumerate() {
                    if let Some(key) = key
                        && *keycode == *key {
                            #[cfg(test)]
                            match (event_type, &unit.state) {
                                (EventType::Pressed, ComboUnitState::None) => {
                                    unit.state =
                                        ComboUnitState::Pending((now, [false; MAX_SOURCES]));
                                    *keycode = KeyCode::None;
                                }
                                (
                                    EventType::Pressed | EventType::Pressing,
                                    &ComboUnitState::Pending(mut state),
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
                                (EventType::Released, &ComboUnitState::Pressing(mut state)) => {
                                    state[i] = false;
                                    if state.iter().all(|&b| !b) {
                                        unit.state = ComboUnitState::None;
                                        *keycode = def.dst;
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
