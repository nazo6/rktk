use rktk_log::error;

use crate::{
    config::CONST_CONFIG,
    config::Hand,
    drivers::interface::{display::DisplayDriver, reporter::Output},
    utils::{Channel, Signal},
};

pub mod default_display;
pub mod utils;

pub enum DisplayMessage {
    Clear,
    Message(&'static str),
    Master(Option<bool>),
    MouseAvailable(bool),
    MouseMove((i8, i8)),
    Output(Output),
    LayerState([bool; CONST_CONFIG.key_manager.layer_count as usize]),
    Hand(Option<Hand>),
    NumLock(bool),
    CapsLock(bool),
    Brightness(u8),
    On(bool),
}

use embedded_graphics::prelude::PixelColor;

#[allow(async_fn_in_trait)]
pub trait DisplayConfig {
    type Color: PixelColor;

    async fn start<D: DisplayDriver<Color = Self::Color>, const N1: usize, const N2: usize>(
        &mut self,
        display: &mut D,
        display_controller: &Channel<DisplayMessage, N1>,
        display_dynamic_message_controller: &Signal<heapless::String<N2>>,
    );
}

pub static DISPLAY_CONTROLLER: Channel<DisplayMessage, 5> = Channel::new();
pub static DISPLAY_DYNAMIC_MESSAGE_CONTROLLER: Signal<heapless::String<256>> = Signal::new();

pub(crate) async fn start<D: DisplayDriver, C: DisplayConfig<Color = D::Color>>(display: &mut D, config: &mut C) {
    if display.init().await.is_err() {
        error!("Failed to initialize display");
        return;
    }

    let _ = display.clear().await;
    let _ = display.flush().await;

    config.start(display, &DISPLAY_CONTROLLER, &DISPLAY_DYNAMIC_MESSAGE_CONTROLLER).await;
}
