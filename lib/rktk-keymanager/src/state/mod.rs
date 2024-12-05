//! Keyboard state management.

#![allow(clippy::let_unit_value)]
#![allow(clippy::single_match)]

use crate::{
    keymap::Keymap,
    time::{Duration, Instant},
};
use config::{
    KeymapInfo, StateConfig, MAX_RESOLVED_KEY_COUNT, MAX_TAP_DANCE_KEY_COUNT,
    MAX_TAP_DANCE_REPEAT_COUNT, ONESHOT_STATE_SIZE,
};
use manager::transparent::TransparentReport;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::state::common::CommonLocalState;

mod common;
pub mod config;
mod key_resolver;
// mod keycode_handler;
mod manager;
mod mouse_handler;

#[derive(Debug)]
pub enum Event {
    Key(KeyChangeEvent),
    Mouse((i8, i8)),
    Encoder((u8, EncoderDirection)),
    None,
}

/// Represents the state of the keyboard.
pub struct State<const LAYER: usize, const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
{
    now: Instant,

    action_handler: key_resolver::KeyResolver<ROW, COL>,

    cs: common::CommonState<LAYER, ROW, COL, ENCODER_COUNT>,
    mouse: manager::mouse::MouseState,
    keyboard: manager::keyboard::KeyboardState,
    media_keyboard: manager::media_keyboard::MediaKeyboardState,
    transparent: manager::transparent::TransparentState,

    config: StateConfig,
}

impl<const LAYER: usize, const ROW: usize, const COL: usize, const ENCODER_COUNT: usize>
    State<LAYER, ROW, COL, ENCODER_COUNT>
{
    /// Creates a new state with the given keymap and configuration.
    pub fn new(layers: Keymap<LAYER, ROW, COL, ENCODER_COUNT>, config: StateConfig) -> Self {
        Self {
            config: config.clone(),
            now: Instant::from_start(Duration::from_millis(0)),
            action_handler: key_resolver::KeyResolver::new(config.key_resolver),

            cs: common::CommonState::new(layers),
            mouse: manager::mouse::MouseState::new(config.mouse),
            keyboard: manager::keyboard::KeyboardState::new(),
            media_keyboard: manager::media_keyboard::MediaKeyboardState::new(),
            transparent: manager::transparent::TransparentState::new(config.initial_output),
        }
    }

    /// Updates state with the given events.
    pub fn update(&mut self, event: Event, since_last_update: Duration) -> StateReport {
        #[cfg(test)]
        dbg!("update");

        self.now = self.now + since_last_update;

        let mut cls = CommonLocalState::new(self.now);

        let mut mls = manager::mouse::MouseLocalState::new(if let Event::Mouse(me) = event {
            me
        } else {
            (0, 0)
        });
        let mut kls = manager::keyboard::KeyboardLocalState::new();
        let mut mkls = manager::media_keyboard::MediaKeyboardLocalState::new();
        let mut tls = manager::transparent::TransparentLocalState::new();

        match event {
            Event::Key(key_change_event) => {
                self.action_handler.resolve_key(
                    &self.cs.keymap,
                    self.cs.layer_active,
                    &key_change_event,
                    self.now,
                    |et, kc| {
                        #[cfg(test)]
                        dbg!(&et, &kc);

                        mls.process_event(&mut self.mouse, &kc, et);
                        kls.process_event(&mut cls, &kc, et);
                        mkls.process_event(&kc);
                        tls.process_event(&mut self.transparent, &kc, et);
                    },
                );
            }
            Event::Mouse(_) => todo!(),
            Event::Encoder(_) => todo!(),
            Event::None => {}
        }

        let highest_layer = self.cs.highest_layer();
        mls.loop_end(&mut self.cs, &mut cls, &mut self.mouse, highest_layer);

        StateReport {
            keyboard_report: kls.report(&cls, &mut self.keyboard),
            mouse_report: mls.report(&mut self.mouse),
            media_keyboard_report: mkls.report(&mut self.media_keyboard),
            transparent_report: tls.report(&mut self.transparent),
            highest_layer: highest_layer as u8,
        }
    }

    pub fn get_keymap(&self) -> &Keymap<LAYER, ROW, COL, ENCODER_COUNT> {
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

/// Information to be communicated to the outside as a result of a state change
#[derive(Debug, PartialEq)]
pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub transparent_report: TransparentReport,
    pub highest_layer: u8,
}

/// Represents a key event.
///
/// Used generically to indicate that the state of a physical key has changed
#[derive(Debug)]
pub struct KeyChangeEvent {
    pub col: u8,
    pub row: u8,
    pub pressed: bool,
}

/// Represents the direction of an encoder
#[derive(Debug)]
pub enum EncoderDirection {
    Clockwise,
    CounterClockwise,
}

#[cfg(test)]
mod tests;
