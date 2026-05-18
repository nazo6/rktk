use crate::{
    interface::state::output_event::EventType,
    keycode::{layer::LayerOp, KeyCode},
};

pub fn update_layer_by_keycode(
    layer_active: &mut [bool],
    keycode: &KeyCode,
    event: EventType,
) {
    match (event, keycode) {
        (EventType::Released, KeyCode::Layer(LayerOp::Momentary(l))) => {
            if let Some(slot) = layer_active.get_mut(*l as usize) {
                *slot = false;
            }
        }
        (_, KeyCode::Layer(LayerOp::Momentary(l))) => {
            if let Some(slot) = layer_active.get_mut(*l as usize) {
                *slot = true;
            }
        }
        (EventType::Pressed, KeyCode::Layer(LayerOp::Toggle(l))) => {
            if let Some(slot) = layer_active.get_mut(*l as usize) {
                *slot = !*slot;
            }
        }
        _ => {}
    };
}
