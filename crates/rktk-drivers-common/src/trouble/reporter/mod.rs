use driver::TroubleReporter;
use rktk::{drivers::interface::wireless::WirelessReporterDriverBuilder, utils::Channel};
use trouble_host::{Controller, gap::PeripheralConfig};

mod driver;
mod server;
mod task;

enum Report {
    Keyboard(usbd_hid::descriptor::KeyboardReport),
    MediaKeyboard(usbd_hid::descriptor::MediaKeyboardReport),
    Mouse(usbd_hid::descriptor::MouseReport),
}

static OUTPUT_CHANNEL: Channel<Report, 4> = Channel::new();

pub struct TroubleReporterConfig {
    pub advertise_name: &'static str,
    pub peripheral_config: Option<PeripheralConfig<'static>>,
}

pub struct TroubleReporterBuilder<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> {
    controller: C,
    config: TroubleReporterConfig,
}

impl<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> TroubleReporterBuilder<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    pub fn new(controller: C, config: TroubleReporterConfig) -> Self {
        Self { controller, config }
    }
}

impl<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> WirelessReporterDriverBuilder
    for TroubleReporterBuilder<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    type Output = TroubleReporter;

    type Error = ();

    async fn build(
        self,
    ) -> Result<(Self::Output, impl Future<Output = ()> + 'static), Self::Error> {
        Ok((
            TroubleReporter { output_tx: OUTPUT_CHANNEL.sender() },
            task::run::<_, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>(
                self.controller,
                OUTPUT_CHANNEL.receiver(),
                self.config,
            ),
        ))
    }
}
