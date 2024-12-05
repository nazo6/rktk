use core::time::Duration;

use crate::{
    keycode::KeyCode,
    state::config::{TapDanceConfig, TapDanceDefinition, MAX_TAP_DANCE_KEY_COUNT},
    time::Instant,
};

use super::EventType;

#[derive(Debug)]
enum TapDanceKeyState {
    None,
    PressedPending {
        tap_count: u8,
        hold_start: Instant,
    },
    ReleasedPending {
        tap_count: u8,
        last_release: Instant,
    },
    Holding {
        tap_count: u8,
    },
}

struct TapDanceUnit {
    state: TapDanceKeyState,
    config: Option<TapDanceDefinition>,
}

impl TapDanceUnit {
    fn get_tap_key(&self, tap_count: u8) -> Option<KeyCode> {
        self.config
            .as_ref()
            .and_then(|def| def.tap.get(tap_count as usize).copied().flatten())
    }

    fn get_hold_key(&self, tap_count: u8) -> Option<KeyCode> {
        self.config
            .as_ref()
            .and_then(|def| def.hold.get(tap_count as usize).copied().flatten())
    }
}

pub struct TapDanceState {
    state: [TapDanceUnit; MAX_TAP_DANCE_KEY_COUNT as usize],
    threshold: Duration,
}

impl TapDanceState {
    pub fn new(config: TapDanceConfig) -> Self {
        Self {
            state: config.definitions.map(|def| TapDanceUnit {
                state: TapDanceKeyState::None,
                config: def,
            }),
            threshold: Duration::from_millis(config.threshold as u64),
        }
    }

    pub fn post_resolve(&mut self, now: Instant, mut cb: impl FnMut(EventType, KeyCode)) {
        for td in &mut self.state {
            match td.state {
                TapDanceKeyState::None => {}
                TapDanceKeyState::PressedPending {
                    tap_count,
                    hold_start,
                } => {
                    // If the Pending state continues for a while (if it remains pressed), it will become Hold.
                    if now - hold_start > self.threshold {
                        if let Some(hkc) = td.get_hold_key(tap_count) {
                            cb(EventType::Pressed, hkc);
                        }
                        td.state = TapDanceKeyState::Holding { tap_count };
                    }
                }
                TapDanceKeyState::ReleasedPending {
                    tap_count,
                    last_release,
                } => {
                    // If the Pending state continues for a while (if it remains released), it will become Tap.
                    if now - last_release > self.threshold {
                        if let Some(tkc) = td.get_tap_key(tap_count) {
                            cb(EventType::Pressed, tkc);
                            cb(EventType::Released, tkc);
                        }
                        td.state = TapDanceKeyState::None;
                    }
                }
                TapDanceKeyState::Holding { tap_count } => {
                    if let Some(hkc) = td.get_tap_key(tap_count) {
                        cb(EventType::Pressing, hkc);
                    }
                }
            }
        }
    }

    pub fn process_event(
        &mut self,
        id: u8,
        now: Instant,
        pressed: bool,
        mut cb: impl FnMut(EventType, KeyCode),
    ) {
        if let Some(td) = self.state.get_mut(id as usize) {
            match (pressed, &td.state) {
                (true, TapDanceKeyState::None) => {
                    td.state = TapDanceKeyState::PressedPending {
                        tap_count: 0,
                        hold_start: now,
                    };
                }
                (false, TapDanceKeyState::Holding { tap_count }) => {
                    if let Some(hkc) = td.get_hold_key(*tap_count) {
                        cb(EventType::Released, hkc);
                    }
                    td.state = TapDanceKeyState::None;
                }
                _ => {}
            }
        }
    }
}
