use rktk::drivers::interface::system::SystemDriver;

pub struct NrfSystemDriver;

impl SystemDriver for NrfSystemDriver {
    fn reset(&self) {
        // not supported
    }

    fn reset_to_bootloader(&self) {
        // not supported
    }
}
