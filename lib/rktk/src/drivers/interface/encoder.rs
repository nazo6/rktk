pub use rktk_keymanager::state::EncoderDirection;

pub trait EncoderDriver {
    async fn read_wait(&mut self) -> (u8, EncoderDirection);
}
