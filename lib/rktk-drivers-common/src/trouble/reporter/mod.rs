use driver::TroubleReporter;
use rand_core::{CryptoRng, RngCore};
use rktk::{drivers::interface::ble::BleDriverBuilder, utils::Channel};
use trouble_host::Controller;

mod driver;
mod server;
mod task;

pub enum Report {
    Keyboard(usbd_hid::descriptor::KeyboardReport),
    MediaKeyboard(usbd_hid::descriptor::MediaKeyboardReport),
    Mouse(usbd_hid::descriptor::MouseReport),
}

static OUTPUT_CHANNEL: Channel<Report, 4> = Channel::new();

pub struct TroubleReporterBuilder<
    C: Controller + 'static,
    RNG: RngCore + CryptoRng + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> {
    controller: C,
    rng: &'static mut RNG,
}

impl<
    C: Controller + 'static,
    RNG: RngCore + CryptoRng,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> TroubleReporterBuilder<C, RNG, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    pub fn new(controller: C, rng: &'static mut RNG) -> Self {
        Self { controller, rng }
    }
}

impl<
    C: Controller + 'static,
    RNG: RngCore + CryptoRng,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> BleDriverBuilder
    for TroubleReporterBuilder<C, RNG, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    type Output = TroubleReporter;

    type Error = ();

    async fn build(
        self,
    ) -> Result<(Self::Output, impl Future<Output = ()> + 'static), Self::Error> {
        Ok((
            TroubleReporter {
                output_tx: OUTPUT_CHANNEL.sender(),
            },
            task::run::<_, _, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>(
                self.controller,
                self.rng,
                OUTPUT_CHANNEL.receiver(),
            ),
        ))
    }
}
