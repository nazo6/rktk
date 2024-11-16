pub use rktk_keymanager::state::EncoderDirection;

pub trait EncoderDriver {
    async fn read_wait(&mut self) -> (u8, EncoderDirection);
}

pub enum DummyEncoderDriver {}
impl EncoderDriver for DummyEncoderDriver {
    async fn read_wait(&mut self) -> (u8, EncoderDirection) {
        unreachable!()
    }
}
