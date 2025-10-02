#![no_std]

use rktk::hooks::{
    AllHooks, Hooks,
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
        hooks: impl AllHooks,
        config: Self::RunConfig,
    );
}
