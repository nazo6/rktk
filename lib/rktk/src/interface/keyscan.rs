#[derive(Debug)]
pub struct KeyChangeEventOneHand {
    pub col: u8,
    pub row: u8,
    pub pressed: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

pub trait KeyscanDriver {
    async fn scan(&mut self) -> heapless::Vec<KeyChangeEventOneHand, 16>;
    async fn current_hand(&mut self) -> Hand;
}
