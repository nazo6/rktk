use crate::{
    keycode::{layer::LayerOp, KeyCode},
    state::key_resolver::EventType,
};

pub fn layer_event_process<const LAYER: usize>(
    layer_active: &mut [bool; LAYER],
    keycode: &KeyCode,
    event: EventType,
) {
    match (event, keycode) {
        (EventType::Released, KeyCode::Layer(LayerOp::Momentary(l))) => {
            layer_active[*l as usize] = false;
        }
        (_, KeyCode::Layer(LayerOp::Momentary(l))) => {
            layer_active[*l as usize] = true;
        }
        (EventType::Pressed, KeyCode::Layer(LayerOp::Toggle(l))) => {
            layer_active[*l as usize] = !layer_active[*l as usize];
        }
        _ => {}
    };
}
