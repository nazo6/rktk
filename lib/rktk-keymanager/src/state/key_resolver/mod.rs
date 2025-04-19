use super::shared::SharedState;
use crate::{
    interface::state::{
        config::KeyResolverConfig, input_event::KeyChangeEvent, output_event::EventType,
    },
    keycode::{KeyAction, KeyCode},
    keymap::{ComboDefinitions, TapDanceDefinitions},
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

    pub fn resolve_key<
        const LAYER: usize,
        const ROW: usize,
        const COL: usize,
        const ENCODER_COUNT: usize,
    >(
        &mut self,
        shared_state: &mut SharedState<
            LAYER,
            ROW,
            COL,
            ENCODER_COUNT,
            TAP_DANCE_MAX_DEFINITIONS,
            TAP_DANCE_MAX_REPEATS,
            COMBO_KEY_MAX_DEFINITIONS,
            COMBO_KEY_MAX_SOURCES,
        >,
        event: Option<&KeyChangeEvent>,
        mut cb: impl FnMut(
            &mut SharedState<
                LAYER,
                ROW,
                COL,
                ENCODER_COUNT,
                TAP_DANCE_MAX_DEFINITIONS,
                TAP_DANCE_MAX_REPEATS,
                COMBO_KEY_MAX_DEFINITIONS,
                COMBO_KEY_MAX_SOURCES,
            >,
            EventType,
            KeyCode,
        ),
    ) {
        let now = shared_state.now;
        macro_rules! with_layer {
            ($cb:expr) => {
                |event_type, mut key_code| {
                    self.combo.process_keycode(&event_type, &mut key_code, now);
                    $cb(shared_state, event_type, key_code);
                }
            };
        }

        {
            let mut cb_with_layer = with_layer!(cb);

            self.oneshot.pre_resolve(event, &mut cb_with_layer);
            self.tap_hold.pre_resolve(event, now, &mut cb_with_layer);

            self.combo.pre_resolve(now, |event_type, key_code| {
                cb(shared_state, event_type, key_code);
            });
        }

        let highest_layer = shared_state.highest_layer();

        if let Some(event) = event {
            let Some(mut key_action) = shared_state
                .keymap
                .get_keyaction(highest_layer, event.row as usize, event.col as usize)
                .copied()
            else {
                return;
            };

            if key_action == KeyAction::Inherit {
                for layer in 0..highest_layer {
                    let Some(action) = shared_state.keymap.get_keyaction(
                        layer,
                        event.row as usize,
                        event.col as usize,
                    ) else {
                        return;
                    };
                    if *action != KeyAction::Inherit {
                        key_action = *action;
                        break;
                    }
                }
            }

            let mut cb_with_layer = with_layer!(cb);

            match key_action {
                KeyAction::Inherit => {}
                KeyAction::Normal(key_code) => {
                    self.normal_state
                        .process_event(event, (key_code, None), &mut cb_with_layer);
                }
                KeyAction::Normal2(key_code, key_code1) => {
                    self.normal_state.process_event(
                        event,
                        (key_code, Some(key_code1)),
                        &mut cb_with_layer,
                    );
                }
                KeyAction::TapHold(tkc, hkc) => {
                    self.tap_hold
                        .process_event(now, event, (tkc, hkc), &mut cb_with_layer);
                }
                KeyAction::OneShot(key_code) => {
                    self.oneshot.process_keycode(&key_code, event.pressed);
                }
                KeyAction::TapDance(id) => {
                    self.tap_dance
                        .process_event(id, now, event.pressed, &mut cb_with_layer);
                }
            }
        }

        let mut cb_with_layer = with_layer!(cb);
        self.tap_dance.post_resolve(now, &mut cb_with_layer);
        self.normal_state.post_resolve(&mut cb_with_layer);
    }
}
