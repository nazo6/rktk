pub trait RandomDriver {
    type Error;
    fn get_random(&self) -> Result<u32, Self::Error>;
}
