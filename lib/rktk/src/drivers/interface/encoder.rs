pub use rktk_keymanager::interface::state::event::EncoderDirection;

pub trait EncoderDriver {
    async fn read_wait(&mut self) -> (u8, EncoderDirection);
}
