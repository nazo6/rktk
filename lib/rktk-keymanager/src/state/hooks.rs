use crate::{interface::state::output_event::EventType, keycode::KeyCode};

pub trait Hooks {
    fn on_key_code(&mut self, _et: EventType, _kc: KeyCode) -> bool {
        true
    }
}

impl<T: Hooks> Hooks for &mut T {
    fn on_key_code(&mut self, et: EventType, kc: KeyCode) -> bool {
        (**self).on_key_code(et, kc)
    }
}

pub struct EmptyHooks;
impl Hooks for EmptyHooks {}
