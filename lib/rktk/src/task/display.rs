use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};

use crate::interface::display::Display;

pub enum DisplayMessage {
    Clear,
    Message(&'static str),
}

pub static DISPLAY_CONTROLLER: Signal<CriticalSectionRawMutex, DisplayMessage> = Signal::new();

pub(super) async fn start<D: Display>(display: D) {
    let mut display = display;
    loop {
        let message = DISPLAY_CONTROLLER.wait().await;
        match message {
            DisplayMessage::Clear => {
                display.clear().await.unwrap();
            }
            DisplayMessage::Message(msg) => {
                display.draw_text_blocking(msg).unwrap();
            }
        }
    }
}
