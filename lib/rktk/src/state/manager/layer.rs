use crate::{
    keycode::{layer::LayerOp, KeyCode},
    state::{common::CommonState, key_resolver::EventType},
};

pub(crate) struct LayerLocalState {}

impl LayerLocalState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_event(
        &mut self,
        common_state: &mut CommonState,
        kc: &KeyCode,
        event: EventType,
    ) {
        match kc {
            KeyCode::Layer(layer_op) => match event {
                EventType::Released => match layer_op {
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
}
