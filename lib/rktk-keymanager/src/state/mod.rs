//! Keyboard state management.

#![allow(clippy::single_match)]

use crate::time::{Duration, Instant};
use config::{
    KeymapInfo, StateConfig, MAX_RESOLVED_KEY_COUNT, MAX_TAP_DANCE_KEY_COUNT,
    MAX_TAP_DANCE_REPEAT_COUNT, ONESHOT_STATE_SIZE,
};
use manager::transparent::TransparentReport;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::{state::common::CommonLocalState, Layer};

mod common;
pub mod config;
mod key_resolver;
mod manager;
mod pressed;

#[derive(Debug, PartialEq)]
pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub transparent_report: TransparentReport,
    pub highest_layer: u8,
}

#[derive(Debug)]
pub struct KeyChangeEvent {
    pub col: u8,
    pub row: u8,
    pub pressed: bool,
}

pub struct State<const LAYER: usize, const ROW: usize, const COL: usize> {
    now: Instant,

    key_resolver: key_resolver::KeyResolver<ROW, COL>,
    pressed: pressed::Pressed<COL, ROW>,

    cs: common::CommonState<LAYER, ROW, COL>,
    mouse: manager::mouse::MouseState,
    keyboard: manager::keyboard::KeyboardState,
    media_keyboard: manager::media_keyboard::MediaKeyboardState,

    config: StateConfig,
}

impl<const LAYER: usize, const ROW: usize, const COL: usize> State<LAYER, ROW, COL> {
    pub fn new(layers: [Layer<ROW, COL>; LAYER], config: StateConfig) -> Self {
        Self {
            config: config.clone(),
            now: Instant::from_start(Duration::from_millis(0)),
            key_resolver: key_resolver::KeyResolver::new(config.key_resolver),
            pressed: pressed::Pressed::new(),

            cs: common::CommonState::new(layers),
            mouse: manager::mouse::MouseState::new(config.mouse),
            keyboard: manager::keyboard::KeyboardState::new(),
            media_keyboard: manager::media_keyboard::MediaKeyboardState::new(),
        }
    }

    /// Updates state with the given events.
    /// If the keyboard is not split, slave_events should be empty.
    pub fn update(
        &mut self,
        key_events: &mut [KeyChangeEvent],
        mouse_event: (i8, i8),
        since_last_update: Duration,
    ) -> StateReport {
        self.now = self.now + since_last_update;

        let mut cls = CommonLocalState::new(self.now);

        let mut mls = manager::mouse::MouseLocalState::new(mouse_event);
        let mut kls = manager::keyboard::KeyboardLocalState::new();
        let mut mkls = manager::media_keyboard::MediaKeyboardLocalState::new();
        let mut tls = manager::transparent::TransparentLocalState::new();

        let events_with_pressing = self.pressed.update_pressed(key_events);
        for (event, kc) in self
            .key_resolver
            .resolve_key(&mut self.cs, &cls, &events_with_pressing)
        {
            if event != key_resolver::EventType::Pressing {
                log::info!("{:?} {:?}", event, kc);
            }

            mls.process_event(&mut self.mouse, &kc, event);
            kls.process_event(&mut cls, &kc, event);
            mkls.process_event(&kc);
            tls.process_event(&kc, event);
        }

        let highest_layer = self.cs.highest_layer();
        mls.loop_end(&mut self.cs, &mut cls, &mut self.mouse, highest_layer);

        StateReport {
            keyboard_report: kls.report(&cls, &mut self.keyboard),
            mouse_report: mls.report(&mut self.mouse),
            media_keyboard_report: mkls.report(&mut self.media_keyboard),
            transparent_report: tls.report(),
            highest_layer: highest_layer as u8,
        }
    }

    pub fn get_keymap(&self) -> &[Layer<ROW, COL>; LAYER] {
        &self.cs.keymap
    }

    pub fn get_config(&self) -> &StateConfig {
        &self.config
    }

    pub fn get_keymap_info() -> KeymapInfo {
        KeymapInfo {
            layer_count: LAYER as u8,
            max_tap_dance_key_count: MAX_TAP_DANCE_KEY_COUNT,
            max_tap_dance_repeat_count: MAX_TAP_DANCE_REPEAT_COUNT,
            oneshot_state_size: ONESHOT_STATE_SIZE,
            max_resolved_key_count: MAX_RESOLVED_KEY_COUNT,
        }
    }
}

#[cfg(test)]
mod tests;
