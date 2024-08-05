use core::fmt::Write as _;

use embassy_futures::select::{select, Either};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};

use crate::interface::{display::DisplayDriver, keyscan::Hand, DriverBuilder};

pub enum DisplayMessage {
    Clear,
    Message(&'static str),
    Master(Option<bool>),
    MouseAvailable(bool),
    MouseMove((i8, i8)),
    HighestLayer(u8),
    Hand(Option<Hand>),
}

pub static DISPLAY_CONTROLLER: Channel<CriticalSectionRawMutex, DisplayMessage, 5> = Channel::new();
pub static DISPLAY_DYNAMIC_MESSAGE_CONTROLLER: Channel<
    CriticalSectionRawMutex,
    heapless::String<256>,
    3,
> = Channel::new();

async fn print_message<D: DisplayDriver>(display: &mut D, msg: &str) {
    display.draw_text("                        ", D::calculate_point(1, 3));
    display.draw_text("                        ", D::calculate_point(2, 3));

    if let Some((l1, l2)) = msg.split_at_checked(20) {
        display.draw_text(l1, D::calculate_point(1, 2));
        display.draw_text(l2, D::calculate_point(1, 3));
    } else {
        display.draw_text(msg, D::calculate_point(1, 2));
    }

    let _ = display.flush_async().await;
}

pub(super) async fn start<D: DisplayDriver>(display_builder: impl DriverBuilder<Output = D>) {
    let Ok(mut display) = display_builder.build().await else {
        panic!("Failed to build display");
    };

    let _ = display
        .update_text("Hello world", D::calculate_point(1, 3))
        .await;
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
                    let _ = print_message(&mut display, msg).await;
                }
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
                                Some(Hand::Left) => "L",
                                Some(Hand::Right) => "R",
                                None => "_",
                            },
                            D::calculate_point(3, 1),
                        )
                        .await;
                }
            },
            Either::Second(str) => {
                let _ = print_message(&mut display, &str).await;
            }
        }
    }
}
