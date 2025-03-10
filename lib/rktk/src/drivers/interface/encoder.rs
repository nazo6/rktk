pub use rktk_keymanager::interface::state::input_event::EncoderDirection;

pub trait EncoderDriver {
    async fn read_wait(&mut self) -> (u8, EncoderDirection);
}
