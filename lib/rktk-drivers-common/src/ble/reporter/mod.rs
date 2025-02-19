use driver::TroubleReporter;
use rktk::drivers::interface::{BackgroundTask, DriverBuilderWithTask};
use task::TroubleReporterTask;
use trouble_host::Controller;

mod driver;
mod server;
mod task;

pub struct TroubleReporterBuilder<C: Controller + 'static> {
    controller: C,
}

impl<C: Controller + 'static> TroubleReporterBuilder<C> {
    pub fn new(controller: C) -> Self {
        Self { controller }
    }
}

impl<C: Controller + 'static> DriverBuilderWithTask for TroubleReporterBuilder<C> {
    type Driver = TroubleReporter;

    type Error = ();

    async fn build(self) -> Result<(Self::Driver, impl BackgroundTask + 'static), Self::Error> {
        Ok((
            TroubleReporter {},
            TroubleReporterTask {
                controller: self.controller,
            },
        ))
    }
}
