pub use rktk_keymanager::interface::state::event::KeyChangeEvent;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hand {
    Left,
    Right,
}

impl Hand {
    pub fn other(&self) -> Hand {
        match self {
            Hand::Left => Hand::Right,
            Hand::Right => Hand::Left,
        }
    }
}

/// Key scanner driver interface.
pub trait KeyscanDriver {
    async fn scan(&mut self, callback: impl FnMut(KeyChangeEvent));
    async fn current_hand(&mut self) -> Hand;
}
