#![no_std]

use rktk::hooks::{
    Hooks,
    interface::{CommonHooks, MasterHooks, RgbHooks, SlaveHooks},
};

pub trait RktkKsp {
    type Irqs: Clone + Copy;
    type Peri;

    type PeriConfig;
    type RunConfig;

    fn init_peripherals(config: Self::PeriConfig) -> Self::Peri;
    async fn start(
        spawner: embassy_executor::Spawner,
        p: Self::Peri,
        hooks: Hooks<impl CommonHooks, impl MasterHooks, impl SlaveHooks, impl RgbHooks>,
        config: Self::RunConfig,
    );
}
