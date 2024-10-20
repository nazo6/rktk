pub enum Pull {
    Up,
    Down,
}

#[allow(async_fn_in_trait)]
pub trait FlexPin {
    fn set_as_input(&mut self);
    fn set_as_output(&mut self);
    fn set_low(&mut self);
    fn set_high(&mut self);
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
    async fn wait_for_high(&mut self);
    async fn wait_for_low(&mut self);
    fn set_pull(&mut self, pull: Pull);
}
