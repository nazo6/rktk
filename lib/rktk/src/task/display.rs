use core::fmt::Write as _;

use embassy_futures::select::{select, Either};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

use crate::interface::{display::Display, keyscan::Hand};

pub enum DisplayMessage {
    Clear,
    Message(&'static str),
    Master(bool),
    MouseAvailable(bool),
    MouseMove((i8, i8)),
    HighestLayer(u8),
    Hand(Hand),
}

pub static DISPLAY_CONTROLLER: Channel<CriticalSectionRawMutex, DisplayMessage, 5> = Channel::new();
pub static DISPLAY_DYNAMIC_MESSAGE_CONTROLLER: Channel<
    CriticalSectionRawMutex,
    heapless::String<256>,
    3,
> = Channel::new();

pub(super) async fn start<D: Display>(mut display: D) {
    let _ = display.init().await;
    loop {
        match select(
            DISPLAY_CONTROLLER.receive(),
            DISPLAY_DYNAMIC_MESSAGE_CONTROLLER.receive(),
        )
        .await
        {
            Either::First(mes) => match mes {
                DisplayMessage::Clear => {
                    display.clear().await.unwrap();
                }
                DisplayMessage::Message(msg) => {
                    let _ = display.update_text(msg, D::calculate_point(1, 3)).await;
                }
                DisplayMessage::Master(master) => {
                    let _ = display
                        .update_text(if master { "M" } else { "S" }, D::calculate_point(1, 1))
                        .await;
                }
                DisplayMessage::MouseAvailable(mouse) => {
                    let _ = display
                        .update_text(if mouse { "m" } else { "x" }, D::calculate_point(2, 1))
                        .await;
                }
                DisplayMessage::MouseMove((x, y)) => {
                    let mut str = heapless::String::<32>::new();
                    write!(str, "[{:3},{:3}]", x, y).unwrap();
                    let _ = display.update_text(&str, D::calculate_point(8, 1)).await;
                }
                DisplayMessage::HighestLayer(layer) => {
                    let mut str = heapless::String::<2>::new();
                    write!(str, "{:1}", layer).unwrap();
                    let _ = display.update_text(&str, D::calculate_point(5, 1)).await;
                }
                DisplayMessage::Hand(hand) => {
                    let _ = display
                        .update_text(
                            match hand {
                                Hand::Left => "L",
                                Hand::Right => "R",
                            },
                            D::calculate_point(3, 1),
                        )
                        .await;
                }
            },
            Either::Second(str) => {
                let _ = display.update_text(&str, D::calculate_point(1, 3)).await;
            }
        }
    }
}
