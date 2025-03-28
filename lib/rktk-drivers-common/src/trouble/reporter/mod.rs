use driver::TroubleReporter;
use rktk::{
    drivers::interface::{BackgroundTask, DriverBuilderWithTask},
    utils::Channel,
};
use task::TroubleReporterTask;
use trouble_host::Controller;

mod driver;
mod server;
mod task;

static OUTPUT_CHANNEL: Channel<usbd_hid::descriptor::KeyboardReport, 4> = Channel::new();

pub struct TroubleReporterBuilder<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> {
    controller: C,
}

impl<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> TroubleReporterBuilder<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    pub fn new(controller: C) -> Self {
        Self { controller }
    }
}

impl<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> DriverBuilderWithTask
    for TroubleReporterBuilder<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    type Driver = TroubleReporter;

    type Error = ();

    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error> {
        Ok((
            TroubleReporter {
                output_tx: OUTPUT_CHANNEL.sender(),
            },
            TroubleReporterTask::<_, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU> {
                controller: self.controller,
                output_rx: OUTPUT_CHANNEL.receiver(),
            },
        ))
    }
}
