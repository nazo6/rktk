use core::fmt::Write as _;

use embassy_futures::select::{select, Either};
use rktk_keymanager::interface::Output;

use crate::{
    drivers::interface::{display::DisplayDriver, keyscan::Hand, DriverBuilder},
    utils::Channel,
};

pub enum DisplayMessage {
    Clear,
    Message(&'static str),
    Master(Option<bool>),
    MouseAvailable(bool),
    MouseMove((i8, i8)),
    Output(Output),
    HighestLayer(u8),
    Hand(Option<Hand>),
    NumLock(bool),
    CapsLock(bool),
}

pub static DISPLAY_CONTROLLER: Channel<DisplayMessage, 5> = Channel::new();
pub static DISPLAY_DYNAMIC_MESSAGE_CONTROLLER: Channel<heapless::String<256>, 3> = Channel::new();

pub(super) async fn start<D: DisplayDriver>(display_builder: impl DriverBuilder<Output = D>) {
    let mut display = match display_builder.build().await {
        Ok(display) => display,
        Err(_e) => {
            log::error!("Failed to initialize display");
            return;
        }
    };
    let _ = display.clear_flush().await;

    loop {
        match select(
            DISPLAY_CONTROLLER.receive(),
            DISPLAY_DYNAMIC_MESSAGE_CONTROLLER.receive(),
        )
        .await
        {
            Either::First(mes) => match mes {
                DisplayMessage::Clear => {
                    display.clear_flush().await.unwrap();
                }
                DisplayMessage::Message(msg) => {
                    let _ = display.print_message(msg).await;
                }

                // (1,1) to (4,1): status
                DisplayMessage::Master(master) => {
                    let _ = display
                        .update_text(
                            match master {
                                Some(true) => "M",
                                Some(false) => "S",
                                None => "_",
                            },
                            D::calculate_point(1, 1),
                        )
                        .await;
                }
                DisplayMessage::MouseAvailable(mouse) => {
                    let _ = display
                        .update_text(if mouse { "m" } else { "x" }, D::calculate_point(2, 1))
                        .await;
                }
                DisplayMessage::Hand(hand) => {
                    let _ = display
                        .update_text(
                            match hand {
                                Some(Hand::Left) => "L",
                                Some(Hand::Right) => "R",
                                None => "_",
                            },
                            D::calculate_point(3, 1),
                        )
                        .await;
                }
                DisplayMessage::Output(output) => {
                    let text = match output {
                        Output::Usb => "U",
                        Output::Ble => "B",
                    };
                    let _ = display.update_text(text, D::calculate_point(4, 1)).await;
                }

                // (6,1): highest layer
                DisplayMessage::HighestLayer(layer) => {
                    let mut str = heapless::String::<2>::new();
                    write!(str, "{:1}", layer).unwrap();
                    let _ = display.update_text(&str, D::calculate_point(6, 1)).await;
                }

                // (8,1): mouse position
                DisplayMessage::MouseMove((x, y)) => {
                    let mut str = heapless::String::<12>::new();
                    write!(str, "[{:3},{:3}]", x, y).unwrap();
                    let _ = display.update_text(&str, D::calculate_point(8, 1)).await;
                }

                // (18,1): num lock
                DisplayMessage::NumLock(num_lock) => {
                    let _ = display
                        .update_text(if num_lock { "N" } else { "n" }, D::calculate_point(18, 1))
                        .await;
                }
                // (19,1): caps lock
                DisplayMessage::CapsLock(caps_lock) => {
                    let _ = display
                        .update_text(if caps_lock { "C" } else { "c" }, D::calculate_point(19, 1))
                        .await;
                }
            },
            Either::Second(str) => {
                let _ = display.print_message(&str).await;
            }
        }
    }
}
