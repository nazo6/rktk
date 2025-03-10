use crate::{
    interface::state::output_event::EventType,
    keycode::{layer::LayerOp, KeyCode},
};

pub fn update_layer_by_keycode<const LAYER: usize>(
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
