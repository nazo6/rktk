use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};

use crate::interface::display::Display;

pub enum DisplayMessage {
    Clear,
    Message(&'static str),
    DynamicMessage(heapless::String<64>),
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
                let _ = display
                    .update_text(msg, embedded_graphics::geometry::Point { x: 0, y: 0 })
                    .await;
            }
            DisplayMessage::DynamicMessage(msg) => {
                let _ = display
                    .update_text(&msg, embedded_graphics::geometry::Point { x: 0, y: 0 })
                    .await;
            }
        }
    }
}
