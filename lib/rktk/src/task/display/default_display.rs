use core::fmt::Write as _;

use embassy_futures::select::{Either, select};
use embedded_graphics::{
    mono_font::{
        MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_6X9, FONT_6X10},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use images::*;

use crate::{
    drivers::interface::{display::DisplayDriver, reporter::Output},
    interface::Hand,
    utils::{Channel, Signal},
};

use super::{DisplayConfig, DisplayMessage};

mod images;

fn get_last_digit_str(n: u8) -> &'static str {
    let digit = n % 10;
    const DIGIT_STRS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    DIGIT_STRS[digit as usize]
}

pub struct DefaultDisplayConfig;
impl DisplayConfig for DefaultDisplayConfig {
    async fn start<D: DisplayDriver, const N1: usize, const N2: usize>(
        &mut self,
        display: &mut D,
        display_controller: &Channel<DisplayMessage, N1>,
        display_dynamic_message_controller: &Signal<heapless::String<N2>>,
    ) {
        loop {
            match select(
                display_controller.receive(),
                display_dynamic_message_controller.wait(),
            )
            .await
            {
                Either::First(mes) => match mes {
                    DisplayMessage::Clear => {
                        display.as_mut().clear(BinaryColor::Off);
                    }
                    DisplayMessage::Message(msg) => {}
                    DisplayMessage::Master(master) => {}
                    DisplayMessage::MouseAvailable(mouse) => {}
                    DisplayMessage::Hand(hand) => {}
                    DisplayMessage::Output(output) => {
                        let image = match output {
                            Output::Usb => IMAGE_USB,
                            Output::Ble => IMAGE_BLUETOOTH,
                        };
                        image.draw(display.as_mut());
                    }
                    DisplayMessage::LayerState(layers) => {
                        for (i, a) in layers.iter().enumerate() {
                            Text::with_baseline(
                                get_last_digit_str(i as u8),
                                Point::new(0, 18 + i as i32 * 9),
                                MonoTextStyleBuilder::new()
                                    .font(&FONT_6X9)
                                    .text_color(if *a {
                                        BinaryColor::Off
                                    } else {
                                        BinaryColor::On
                                    })
                                    .background_color(if *a {
                                        BinaryColor::On
                                    } else {
                                        BinaryColor::Off
                                    })
                                    .build(),
                                Baseline::Top,
                            )
                            .draw(display.as_mut());
                        }
                    }
                    DisplayMessage::MouseMove((x, y)) => {}
                    DisplayMessage::NumLock(num_lock) => {}
                    DisplayMessage::CapsLock(caps_lock) => {}
                    DisplayMessage::Brightness(brightness) => {
                        let _ = display.set_brightness(brightness).await;
                    }
                    DisplayMessage::On(on) => {
                        let _ = display.set_display_on(on).await;
                    }
                },
                Either::Second(str) => {}
            }

            display.flush().await;
        }
    }
}
