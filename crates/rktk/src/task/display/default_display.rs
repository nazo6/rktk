use embassy_futures::select::{Either, select};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_8X13},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use images::*;

use crate::{
    drivers::interface::{display::DisplayDriver, reporter::Output},
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
                        let _ = display.as_mut().clear(BinaryColor::Off);
                    }
                    DisplayMessage::Message(_msg) => {
                        // TODO: Implement this
                    }
                    DisplayMessage::Output(output) => {
                        let image = match output {
                            Output::Usb => IMAGE_USB,
                            Output::Ble => IMAGE_BLUETOOTH,
                        };
                        let _ = image.translate(Point::new(8, 0)).draw(display.as_mut());
                    }
                    DisplayMessage::LayerState(layers) => {
                        for (i, a) in layers.iter().enumerate() {
                            let _ = Text::with_baseline(
                                get_last_digit_str(i as u8),
                                Point::new(0, 20 + i as i32 * 13),
                                MonoTextStyleBuilder::new()
                                    .font(&FONT_8X13)
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
                    DisplayMessage::MouseAvailable(mouse) => {
                        if mouse {
                            let _ = IMAGE_MOUSE
                                .translate(Point::new(16, 35))
                                .draw(display.as_mut());
                        }
                    }
                    DisplayMessage::NumLock(num_lock) => {
                        let _ = Text::with_baseline(
                            "N",
                            Point::new(14, 20),
                            MonoTextStyleBuilder::new()
                                .font(&FONT_8X13)
                                .text_color(if num_lock {
                                    BinaryColor::Off
                                } else {
                                    BinaryColor::On
                                })
                                .background_color(if num_lock {
                                    BinaryColor::On
                                } else {
                                    BinaryColor::Off
                                })
                                .build(),
                            Baseline::Top,
                        )
                        .draw(display.as_mut());
                    }
                    DisplayMessage::CapsLock(caps_lock) => {
                        let _ = Text::with_baseline(
                            "C",
                            Point::new(24, 20),
                            MonoTextStyleBuilder::new()
                                .font(&FONT_8X13)
                                .text_color(if caps_lock {
                                    BinaryColor::Off
                                } else {
                                    BinaryColor::On
                                })
                                .background_color(if caps_lock {
                                    BinaryColor::On
                                } else {
                                    BinaryColor::Off
                                })
                                .build(),
                            Baseline::Top,
                        )
                        .draw(display.as_mut());
                    }
                    DisplayMessage::Brightness(brightness) => {
                        let _ = display.set_brightness(brightness).await;
                    }
                    DisplayMessage::On(on) => {
                        let _ = display.set_display_on(on).await;
                    }
                    _ => {}
                },
                Either::Second(_str) => {
                    // TODO: Implement this
                }
            }

            let _ = display.flush().await;
        }
    }
}
