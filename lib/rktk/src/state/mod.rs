//! Keyboard state management.

#![allow(clippy::single_match)]

use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use crate::{
    config::static_config::CONFIG,
    interface::keyscan::{Hand, KeyChangeEventOneHand},
    keycode::Layer,
    state::common::CommonLocalState,
};

mod common;
mod key_resolver;
mod manager;
mod pressed;

pub struct StateReport {
    pub keyboard_report: Option<KeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub highest_layer: u8,
}

pub struct State {
    /// Specifies which hand is the master when the keyboard is split.
    /// If None, the keyboard is not split.
    master_hand: Option<Hand>,

    pressed: pressed::Pressed,
    key_resolver: key_resolver::KeyResolver,

    cs: common::CommonState,
    mouse: manager::mouse::MouseState,
    keyboard: manager::keyboard::KeyboardState,
    media_keyboard: manager::media_keyboard::MediaKeyboardState,
}

impl State {
    pub fn new(layers: [Layer; CONFIG.layer_count], master_hand: Option<Hand>) -> Self {
        Self {
            master_hand,

            pressed: pressed::Pressed::new(),
            key_resolver: key_resolver::KeyResolver::new(),

            cs: common::CommonState::new(layers),
            mouse: manager::mouse::MouseState::new(),
            keyboard: manager::keyboard::KeyboardState::new(),
            media_keyboard: manager::media_keyboard::MediaKeyboardState::new(),
        }
    }

    /// Updates state with the given events.
    /// If the keyboard is not split, slave_events should be empty.
    #[inline(always)]
    pub fn update(
        &mut self,
        master_events: &mut [KeyChangeEventOneHand],
        slave_events: &mut [KeyChangeEventOneHand],
        mouse_event: (i8, i8),
    ) -> StateReport {
        let prev_highest_layer = self.cs.highest_layer();

        let mut cls = CommonLocalState::new(prev_highest_layer);

        let mut mls = manager::mouse::MouseLocalState::new(mouse_event);
        let mut kls = manager::keyboard::KeyboardLocalState::new();
        let mut mkls = manager::media_keyboard::MediaKeyboardLocalState::new();
        let mut lls = manager::layer::LayerLocalState::new();

        let events = self.pressed.compose_events_and_update_pressed(
            self.master_hand,
            master_events,
            slave_events,
        );

        for (event, kc) in self.key_resolver.resolve_key(&mut self.cs, &cls, &events) {
            mls.process_event(&mut self.mouse, &kc, event);
            kls.process_event(&mut cls, &kc, event);
            mkls.process_event(&kc);
            lls.process_event(&mut self.cs, &kc, event);
        }

        mls.loop_end(&mut self.cs, &mut cls, &mut self.mouse);

        StateReport {
            keyboard_report: kls.report(&cls, &mut self.keyboard),
            mouse_report: mls.report(&mut self.mouse),
            media_keyboard_report: mkls.report(&mut self.media_keyboard),
            highest_layer: prev_highest_layer as u8,
        }
    }
}
