use crate::{
    interface::state::{
        config::KeyResolverConfig, input_event::KeyChangeEvent, output_event::EventType,
    },
    keycode::{KeyAction, KeyCode},
    keymap::{ComboDefinitions, TapDanceDefinitions},
    time::Instant,
};

mod combo;
mod normal;
mod oneshot;
mod tap_dance;
mod tap_hold;

/// Handles layer related events and resolve physical key position to keycode.
pub struct KeyResolver<
    const NORMAL_MAX_PRESSED_KEYS: usize,
    const ONESHOT_BUFFER_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
> {
    normal_state: normal::NormalState<NORMAL_MAX_PRESSED_KEYS>,
    tap_dance: tap_dance::TapDanceState<TAP_DANCE_MAX_DEFINITIONS, TAP_DANCE_MAX_REPEATS>,
    oneshot: oneshot::OneshotState<ONESHOT_BUFFER_SIZE>,
    tap_hold: tap_hold::TapHoldState,
    combo: combo::ComboState<COMBO_KEY_MAX_DEFINITIONS, COMBO_KEY_MAX_SOURCES>,
}

impl<
    const NORMAL_MAX_PRESSED_KEYS: usize,
    const ONESHOT_BUFFER_SIZE: usize,
    const TAP_DANCE_MAX_DEFINITIONS: usize,
    const TAP_DANCE_MAX_REPEATS: usize,
    const COMBO_KEY_MAX_DEFINITIONS: usize,
    const COMBO_KEY_MAX_SOURCES: usize,
>
    KeyResolver<
        NORMAL_MAX_PRESSED_KEYS,
        ONESHOT_BUFFER_SIZE,
        TAP_DANCE_MAX_DEFINITIONS,
        TAP_DANCE_MAX_REPEATS,
        COMBO_KEY_MAX_DEFINITIONS,
        COMBO_KEY_MAX_SOURCES,
    >
{
    pub fn new(
        config: KeyResolverConfig,
        tap_dance_def: TapDanceDefinitions<TAP_DANCE_MAX_DEFINITIONS, TAP_DANCE_MAX_REPEATS>,
        combo_def: ComboDefinitions<COMBO_KEY_MAX_DEFINITIONS, COMBO_KEY_MAX_SOURCES>,
    ) -> Self {
        Self {
            normal_state: normal::NormalState::new(),
            tap_dance: tap_dance::TapDanceState::new(tap_dance_def, config.tap_dance),
            oneshot: oneshot::OneshotState::new(),
            tap_hold: tap_hold::TapHoldState::new(config.tap_hold),
            combo: combo::ComboState::new(combo_def, config.combo),
        }
    }

    pub fn resolve_key(
        &mut self,
        keymap: &impl crate::keymap::KeymapLookup,
        active_layers: &mut [bool],
        now: Instant,
        event: Option<&KeyChangeEvent>,
    ) -> heapless::Vec<(KeyCode, EventType), 16> {
        let mut out = heapless::Vec::new();

        // 1. Pre-resolve phase
        // Oneshot, TapHold and Combo states resolve their pending/time-dependent states.

        // Temporarily collect sub-stage events to run them through combo processing
        let mut sub_events = heapless::Vec::new();
        self.oneshot.pre_resolve(event, &mut sub_events);
        self.tap_hold.pre_resolve(event, now, &mut sub_events);

        for (kc, et) in sub_events {
            let mut keycode = kc;
            self.combo.process_keycode(&et, &mut keycode, now);
            if !matches!(keycode, KeyCode::None) {
                super::updater::layer::update_layer_by_keycode(active_layers, &keycode, et);
                let _ = out.push((keycode, et));
            }
        }

        // Combo pre_resolve events bypass combo filtering
        let mut combo_events = heapless::Vec::new();
        self.combo.pre_resolve(now, &mut combo_events);
        for (kc, et) in combo_events {
            super::updater::layer::update_layer_by_keycode(active_layers, &kc, et);
            let _ = out.push((kc, et));
        }

        // Compute highest layer dynamically
        let mut highest_layer = 0;
        for (l, &active) in active_layers.iter().enumerate() {
            if active {
                highest_layer = l;
            }
        }

        // 2. Direct event processing phase
        if let Some(event) = event {
            let Some(mut key_action) = keymap
                .get_keyaction(highest_layer, event.row as usize, event.col as usize)
                .copied()
            else {
                return out;
            };

            if key_action == KeyAction::Inherit {
                for layer in (0..highest_layer).rev() {
                    if let Some(action) = keymap.get_keyaction(
                        layer,
                        event.row as usize,
                        event.col as usize,
                    )
                        && *action != KeyAction::Inherit
                    {
                        key_action = *action;
                        break;
                    }
                }
            }

            let mut process_events = heapless::Vec::new();
            match key_action {
                KeyAction::Inherit => {}
                KeyAction::Normal(_) | KeyAction::Normal2(_, _) => {
                    self.normal_state.process_event(
                        event,
                        key_action,
                        highest_layer,
                        keymap,
                        &mut process_events,
                    );
                }
                KeyAction::TapHold(tkc, hkc) => {
                    self.tap_hold
                        .process_event(now, event, (tkc, hkc), &mut process_events);
                }
                KeyAction::OneShot(key_code) => {
                    self.oneshot.process_keycode(&key_code, event.pressed);
                }
                KeyAction::TapDance(id) => {
                    self.tap_dance
                        .process_event(id, now, event.pressed, &mut process_events);
                }
            }

            for (kc, et) in process_events {
                let mut keycode = kc;
                self.combo.process_keycode(&et, &mut keycode, now);
                if !matches!(keycode, KeyCode::None) {
                    super::updater::layer::update_layer_by_keycode(active_layers, &keycode, et);
                    let _ = out.push((keycode, et));
                }
            }
        }

        // 3. Post-resolve phase
        let mut post_events = heapless::Vec::new();
        self.tap_dance.post_resolve(now, &mut post_events);
        self.normal_state.post_resolve(keymap, &mut post_events);

        for (kc, et) in post_events {
            let mut keycode = kc;
            self.combo.process_keycode(&et, &mut keycode, now);
            if !matches!(keycode, KeyCode::None) {
                super::updater::layer::update_layer_by_keycode(active_layers, &keycode, et);
                let _ = out.push((keycode, et));
            }
        }

        out
    }
}

