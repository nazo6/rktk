use nrf_softdevice::{random_bytes, Softdevice};
use rktk::drivers::interface::rand::RandomDriver;

pub struct SdRand {
    sd: &'static Softdevice,
}

impl SdRand {
    pub fn new(sd: &'static Softdevice) -> Self {
        Self { sd }
    }
}

impl RandomDriver for SdRand {
    type Error = ();
    fn get_random(&self) -> Result<u32, Self::Error> {
        let mut buf = [0u8; 4];
        let Ok(_) = random_bytes(self.sd, &mut buf) else {
            return Err(());
        };

        Ok(u32::from_le_bytes(buf))
    }
}
