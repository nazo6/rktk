use crate::{
    keycode::{layer::LayerOp, KeyCode},
    state::{
        common::{CommonLocalState, CommonState},
        pressed::{KeyStatusChangeType, KeyStatusUpdateEvent},
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
        event: &KeyStatusUpdateEvent,
    ) {
        match kc {
            KeyCode::Layer(layer_op) => match event.change_type {
                KeyStatusChangeType::Released(_) => match layer_op {
                    LayerOp::Move(l) => {
                        common_state.layer_active[*l] = false;
                    }
                    LayerOp::Toggle(l) => {
                        common_state.layer_active[*l] = !common_state.layer_active[*l];
                    }
                },
                _ => match layer_op {
                    LayerOp::Move(l) => {
                        common_state.layer_active[*l] = true;
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
