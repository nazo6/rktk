use crate::{
    keycode::{layer::LayerOp, KeyCode},
    state::{
        common::{CommonLocalState, CommonState},
        pressed::{KeyStatus, KeyStatusEvent},
    },
};

use super::interface::LocalStateManager;

pub struct LayerLocalState {}

impl LayerLocalState {
    pub fn new() -> Self {
        Self {}
    }
}

impl LocalStateManager for LayerLocalState {
    type GlobalState = ();
    type Report = ();

    fn process_event(
        &mut self,
        common_state: &mut CommonState,
        _common_local_state: &mut CommonLocalState,
        _global_state: &mut Self::GlobalState,
        kc: &KeyCode,
        event: &KeyStatusEvent,
    ) {
        match kc {
            KeyCode::Layer(layer_op) => match event.change_type {
                KeyStatus::Released(_) => match layer_op {
                    LayerOp::Move(l) => {
                        common_state.layer_active[*l as usize] = false;
                    }
                    LayerOp::Toggle(l) => {
                        common_state.layer_active[*l as usize] =
                            !common_state.layer_active[*l as usize];
                    }
                },
                _ => match layer_op {
                    LayerOp::Move(l) => {
                        common_state.layer_active[*l as usize] = true;
                    }
                    _ => {}
                },
            },
            _ => {}
        };
    }

    fn report(
        self,
        _common_state: &CommonState,
        _common_local_state: &CommonLocalState,
        _global_state: &mut Self::GlobalState,
    ) -> Option<Self::Report> {
        None
    }
}
