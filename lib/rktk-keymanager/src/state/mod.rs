//! Keyboard state management.

#![allow(clippy::single_match)]

use embassy_time::{Duration, Instant};
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::{state::common::CommonLocalState, Layer};

mod common;
mod key_resolver;
mod manager;
mod pressed;

#[derive(Debug, PartialEq)]
pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub highest_layer: u8,
}

#[derive(Debug)]
pub struct KeyChangeEvent {
    pub col: u8,
    pub row: u8,
    pub pressed: bool,
}

pub struct StateConfig {
    pub tap_threshold: Duration,
    pub auto_mouse_layer: usize,
    pub auto_mouse_duration: embassy_time::Duration,
    pub auto_mouse_threshold: u8,
    pub scroll_divider_x: i8,
    pub scroll_divider_y: i8,
}

pub struct State<const LAYER: usize, const ROW: usize, const COL: usize> {
    key_resolver: key_resolver::KeyResolver<ROW, COL>,
    pressed: pressed::Pressed<COL, ROW>,

    cs: common::CommonState<LAYER, ROW, COL>,
    mouse: manager::mouse::MouseState,
    keyboard: manager::keyboard::KeyboardState,
    media_keyboard: manager::media_keyboard::MediaKeyboardState,
}

impl<const LAYER: usize, const ROW: usize, const COL: usize> State<LAYER, ROW, COL> {
    pub fn new(layers: [Layer<ROW, COL>; LAYER], config: StateConfig) -> Self {
        Self {
            key_resolver: key_resolver::KeyResolver::new(config.tap_threshold),
            pressed: pressed::Pressed::new(),

            cs: common::CommonState::new(layers),
            mouse: manager::mouse::MouseState::new(
                config.auto_mouse_layer,
                config.auto_mouse_duration,
                config.auto_mouse_threshold,
                config.scroll_divider_x,
                config.scroll_divider_y,
            ),
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
        now: Instant,
    ) -> StateReport {
        let mut cls = CommonLocalState::new(now);

        let mut mls = manager::mouse::MouseLocalState::new(mouse_event);
        let mut kls = manager::keyboard::KeyboardLocalState::new();
        let mut mkls = manager::media_keyboard::MediaKeyboardLocalState::new();

        let events_with_pressing = self.pressed.update_pressed(key_events);
        for (event, kc) in self
            .key_resolver
            .resolve_key(&mut self.cs, &cls, &events_with_pressing)
        {
            mls.process_event(&mut self.mouse, &kc, event);
            kls.process_event(&mut cls, &kc, event);
            mkls.process_event(&kc);
        }

        let highest_layer = self.cs.highest_layer();
        mls.loop_end(&mut self.cs, &mut cls, &mut self.mouse, highest_layer);

        StateReport {
            keyboard_report: kls.report(&cls, &mut self.keyboard),
            mouse_report: mls.report(&mut self.mouse),
            media_keyboard_report: mkls.report(&mut self.media_keyboard),
            highest_layer: highest_layer as u8,
        }
    }

    pub fn get_keymap_mut(&mut self) -> &mut [Layer<ROW, COL>; LAYER] {
        &mut self.cs.keymap
    }
}

#[cfg(test)]
mod tests;
