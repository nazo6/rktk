#![allow(async_fn_in_trait)]

use crate::drivers::interface::{
    backlight::{BacklightCommand, BacklightDriver},
    keyscan::{Hand, KeyscanDriver},
    mouse::MouseDriver,
    // reporter::ReporterDriver,
    storage::StorageDriver,
};
pub use crate::task::main_loop::{M2sTx, S2mTx};

pub struct Hooks<MH: MainHooks, BH: BacklightHooks> {
    pub main: MH,
    pub backlight: BH,
}

pub trait MainHooks {
    async fn on_init(
        &mut self,
        _hand: Hand,
        _key_scanner: &mut impl KeyscanDriver,
        _mouse: Option<&mut impl MouseDriver>,
        // _reporter: Option<&impl ReporterDriver>,
        _storage: Option<&mut impl StorageDriver>,
    ) {
    }
    async fn on_master_init(
        &mut self,
        _key_scanner: &mut impl KeyscanDriver,
        _mouse: Option<&mut impl MouseDriver>,
        // _reporter: &impl ReporterDriver,
        _to_slave: &M2sTx<'_>,
    ) {
    }
    async fn on_slave_init(
        &mut self,
        _key_scanner: &mut impl KeyscanDriver,
        _mouse: Option<&mut impl MouseDriver>,
        _to_slave: &S2mTx<'_>,
    ) {
    }
}

pub use smart_leds::RGB8;

pub trait BacklightHooks {
    async fn on_backlight_init(&mut self, _driver: &mut impl BacklightDriver) {}
    async fn on_backlight_process<const N: usize>(
        &mut self,
        _driver: &mut impl BacklightDriver,
        _command: &BacklightCommand,
        _rgb_data: &mut Option<[RGB8; N]>,
    ) {
    }
}

pub const fn create_empty_hooks() -> Hooks<EmptyMainHooks, EmptyBacklightHooks> {
    Hooks {
        main: EmptyMainHooks,
        backlight: EmptyBacklightHooks,
    }
}

pub struct EmptyMainHooks;
impl MainHooks for EmptyMainHooks {}
pub struct EmptyBacklightHooks;
impl BacklightHooks for EmptyBacklightHooks {}
