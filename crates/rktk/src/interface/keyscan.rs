#[derive(Debug)]
pub struct KeyChangeEventOneHand {
    pub col: u8,
    pub row: u8,
    pub pressed: bool,
}

pub trait Keyscan {
    async fn scan(&mut self) -> heapless::Vec<KeyChangeEventOneHand, 16>;
}
