use embassy_futures::select::{Either3, select3};
use embassy_time::{Duration, Ticker, Timer};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::{FONT_6X10, FONT_8X13, FONT_9X15}},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::{Rectangle, RoundedRectangle, CornerRadii, PrimitiveStyleBuilder, Circle, Line},
    text::{Baseline, Text},
};

use crate::{
    config::CONST_CONFIG,
    drivers::interface::{display::DisplayDriver, reporter::Output},
    utils::{Channel, Signal},
};

use super::{DisplayConfig, DisplayMessage};

// 32-element sine lookup table scaled to 100 to avoid float operations
const SIN_TABLE: [i32; 32] = [
    0, 19, 38, 55, 70, 83, 92, 98,
    100, 98, 92, 83, 70, 55, 38, 19,
    0, -19, -38, -55, -70, -83, -92, -98,
    -100, -98, -92, -83, -70, -55, -38, -19
];

#[inline]
fn get_sin(idx: u32) -> i32 {
    SIN_TABLE[(idx % 32) as usize]
}

#[inline]
fn rgb(r: u8, g: u8, b: u8) -> Rgb565 {
    Rgb565::from(Rgb888::new(r, g, b))
}

fn draw_centered_text<D: DrawTarget<Color = Rgb565>>(
    target: &mut D,
    text: &str,
    font: &embedded_graphics::mono_font::MonoFont,
    rect: Rectangle,
    text_color: Rgb565,
) -> Result<(), D::Error> {
    let text_width = text.len() as i32 * font.character_size.width as i32;
    let text_height = font.character_size.height as i32;
    let x = rect.top_left.x + (rect.size.width as i32 - text_width) / 2;
    let y = rect.top_left.y + (rect.size.height as i32 - text_height) / 2;
    
    let text_style = MonoTextStyleBuilder::new()
        .font(font)
        .text_color(text_color)
        .build();
        
    Text::with_baseline(text, Point::new(x, y), text_style, Baseline::Top).draw(target)?;
    Ok(())
}

async fn draw_dashboard<D: DisplayDriver<Color = Rgb565>>(
    display: &mut D,
    layer_state: &[bool],
    caps_lock: bool,
    num_lock: bool,
    output_mode: Output,
    mouse_available: bool,
    anim_tick: u32,
) {
    let target = display.draw_target();

    // 1. Draw Background (Synthwave Dark Slate)
    let bg_style = PrimitiveStyleBuilder::new()
        .fill_color(rgb(10, 10, 15))
        .build();
    let _ = Rectangle::new(Point::zero(), Size::new(284, 76)).into_styled(bg_style).draw(target);

    // Outer framing styling
    let panel_border_style = PrimitiveStyleBuilder::new()
        .stroke_color(rgb(0, 180, 216))
        .stroke_width(1)
        .build();

    // 2. Left Panel: X = 6, Y = 6, W = 66, H = 64
    let _ = RoundedRectangle::new(
        Rectangle::new(Point::new(6, 6), Size::new(66, 64)),
        CornerRadii::new(Size::new(6, 6)),
    )
    .into_styled(panel_border_style)
    .draw(target);

    // Left Panel - RKTK branding
    let brand_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X15)
        .text_color(rgb(0, 180, 216))
        .build();
    let _ = Text::with_baseline("RKTK", Point::new(18, 12), brand_style, Baseline::Top).draw(target);

    // Divider line under branding
    let line_style = PrimitiveStyleBuilder::new()
        .stroke_color(rgb(0, 70, 90))
        .stroke_width(1)
        .build();
    let _ = Line::new(Point::new(14, 29), Point::new(64, 29)).into_styled(line_style).draw(target);

    // Left Panel - Connection Badge
    let (conn_text, conn_bg, conn_fg) = match output_mode {
        Output::Usb => ("USB", rgb(0, 60, 20), rgb(0, 255, 100)),
        Output::Ble => ("BLE", rgb(0, 30, 80), rgb(0, 180, 255)),
    };
    let badge_rect = Rectangle::new(Point::new(12, 34), Size::new(54, 16));
    let badge_style = PrimitiveStyleBuilder::new()
        .fill_color(conn_bg)
        .build();
    let _ = RoundedRectangle::new(badge_rect, CornerRadii::new(Size::new(4, 4)))
        .into_styled(badge_style)
        .draw(target);
    let _ = draw_centered_text(target, conn_text, &FONT_6X10, badge_rect, conn_fg);

    // Battery / Status bar
    let bat_border_style = PrimitiveStyleBuilder::new()
        .stroke_color(rgb(60, 70, 80))
        .stroke_width(1)
        .build();
    let _ = Rectangle::new(Point::new(14, 55), Size::new(32, 6))
        .into_styled(bat_border_style)
        .draw(target);

    let bat_fill_style = PrimitiveStyleBuilder::new()
        .fill_color(if mouse_available { rgb(0, 220, 100) } else { rgb(0, 200, 255) })
        .build();
    let _ = Rectangle::new(Point::new(15, 56), Size::new(26, 4))
        .into_styled(bat_fill_style)
        .draw(target);

    // 3. Center Panel: X = 78, Y = 6, W = 128, H = 64
    let center_border_style = PrimitiveStyleBuilder::new()
        .stroke_color(rgb(40, 50, 70))
        .stroke_width(1)
        .build();
    let _ = RoundedRectangle::new(
        Rectangle::new(Point::new(78, 6), Size::new(128, 64)),
        CornerRadii::new(Size::new(6, 6)),
    )
    .into_styled(center_border_style)
    .draw(target);

    let label_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(rgb(120, 130, 150))
        .build();
    let _ = Text::with_baseline("ACTIVE LAYER", Point::new(86, 12), label_style, Baseline::Top).draw(target);

    // Determine Active Layer
    let active_layer = layer_state.iter().position(|&x| x).unwrap_or(0);
    let (layer_name, layer_color) = match active_layer {
        0 => ("BASE", rgb(0, 100, 150)),
        1 => ("LOWER", rgb(180, 80, 0)),
        2 => ("RAISE", rgb(130, 0, 180)),
        3 => ("NAV", rgb(0, 120, 40)),
        4 => ("MEDIA", rgb(180, 0, 80)),
        _ => ("LAYER X", rgb(60, 60, 60)),
    };

    let layer_badge_rect = Rectangle::new(Point::new(86, 24), Size::new(112, 20));
    let layer_badge_style = PrimitiveStyleBuilder::new()
        .fill_color(layer_color)
        .build();
    let _ = RoundedRectangle::new(layer_badge_rect, CornerRadii::new(Size::new(4, 4)))
        .into_styled(layer_badge_style)
        .draw(target);
    let _ = draw_centered_text(target, layer_name, &FONT_8X13, layer_badge_rect, rgb(255, 255, 255));

    // Lock indicators (CAPS & NUM)
    // CAPS
    let caps_rect = Rectangle::new(Point::new(86, 49), Size::new(52, 14));
    if caps_lock {
        let caps_active_style = PrimitiveStyleBuilder::new()
            .fill_color(rgb(220, 0, 100))
            .build();
        let _ = RoundedRectangle::new(caps_rect, CornerRadii::new(Size::new(3, 3)))
            .into_styled(caps_active_style)
            .draw(target);
        let _ = draw_centered_text(target, "CAPS", &FONT_6X10, caps_rect, rgb(255, 255, 255));
    } else {
        let caps_inactive_style = PrimitiveStyleBuilder::new()
            .stroke_color(rgb(40, 50, 60))
            .stroke_width(1)
            .build();
        let _ = RoundedRectangle::new(caps_rect, CornerRadii::new(Size::new(3, 3)))
            .into_styled(caps_inactive_style)
            .draw(target);
        let _ = draw_centered_text(target, "CAPS", &FONT_6X10, caps_rect, rgb(80, 90, 100));
    }

    // NUM
    let num_rect = Rectangle::new(Point::new(146, 49), Size::new(52, 14));
    if num_lock {
        let num_active_style = PrimitiveStyleBuilder::new()
            .fill_color(rgb(0, 180, 100))
            .build();
        let _ = RoundedRectangle::new(num_rect, CornerRadii::new(Size::new(3, 3)))
            .into_styled(num_active_style)
            .draw(target);
        let _ = draw_centered_text(target, "NUM", &FONT_6X10, num_rect, rgb(255, 255, 255));
    } else {
        let num_inactive_style = PrimitiveStyleBuilder::new()
            .stroke_color(rgb(40, 50, 60))
            .stroke_width(1)
            .build();
        let _ = RoundedRectangle::new(num_rect, CornerRadii::new(Size::new(3, 3)))
            .into_styled(num_inactive_style)
            .draw(target);
        let _ = draw_centered_text(target, "NUM", &FONT_6X10, num_rect, rgb(80, 90, 100));
    }

    // 4. Right Panel: X = 212, Y = 6, W = 66, H = 64
    let _ = RoundedRectangle::new(
        Rectangle::new(Point::new(212, 6), Size::new(66, 64)),
        CornerRadii::new(Size::new(6, 6)),
    )
    .into_styled(panel_border_style)
    .draw(target);

    // "LIVE" label
    let _ = Text::with_baseline("LIVE", Point::new(220, 12), label_style, Baseline::Top).draw(target);

    // Blinking red recording dot
    let dot_active = (anim_tick % 20) < 10;
    let dot_style = PrimitiveStyleBuilder::new()
        .fill_color(if dot_active { rgb(255, 0, 50) } else { rgb(80, 0, 10) })
        .build();
    let _ = Circle::new(Point::new(254, 14), 5).into_styled(dot_style).draw(target);

    // Animated bouncing audio-style visualizer bars
    let bar_color = match active_layer {
        0 => rgb(0, 200, 255),
        1 => rgb(255, 120, 0),
        2 => rgb(200, 0, 255),
        3 => rgb(0, 255, 100),
        4 => rgb(255, 0, 120),
        _ => rgb(150, 150, 150),
    };

    let bar_fill_style = PrimitiveStyleBuilder::new()
        .fill_color(bar_color)
        .build();

    // Draw 8 bars representing dynamic visualizer
    // Start X = 222, spacing = 6 (width 4 + gap 2), bottom Y = 60
    for i in 0..8 {
        let phase1 = (anim_tick.wrapping_mul(2).wrapping_add(i * 3)) % 32;
        let phase2 = (anim_tick.wrapping_add(i * 7)) % 32;
        let val = (get_sin(phase1) + get_sin(phase2)) / 2; // -100 to 100
        
        // Bar height range: 4 to 34 pixels
        let bar_height = 4 + ((val + 100) * 30) / 200;
        
        let x = 222 + (i as i32 * 6);
        let y = 60 - bar_height;
        
        let _ = Rectangle::new(Point::new(x, y), Size::new(4, bar_height as u32))
            .into_styled(bar_fill_style)
            .draw(target);
    }
}

pub struct ColorBarDisplayConfig;

impl DisplayConfig for ColorBarDisplayConfig {
    type Color = Rgb565;

    async fn start<D: DisplayDriver<Color = Self::Color>, const N1: usize, const N2: usize>(
        &mut self,
        display: &mut D,
        display_controller: &Channel<DisplayMessage, N1>,
        display_dynamic_message_controller: &Signal<heapless::String<N2>>,
    ) {
        let mut layer_state = [false; CONST_CONFIG.key_manager.layer_count as usize];
        layer_state[0] = true; // BASE layer is active initially
        let mut caps_lock = false;
        let mut num_lock = false;
        let mut output_mode = Output::Usb;
        let mut mouse_available = false;
        let mut anim_tick = 0u32;
        let mut anim_ticker = Ticker::every(Duration::from_millis(50));

        // Render initial static screen
        let _ = display.clear().await;
        draw_dashboard(
            display,
            &layer_state,
            caps_lock,
            num_lock,
            output_mode,
            mouse_available,
            anim_tick,
        )
        .await;
        let _ = display.flush().await;

        loop {
            // Check for incoming updates or tick the animation timer (50ms interval ~ 20 FPS)
            let select_res = select3(
                display_controller.receive(),
                display_dynamic_message_controller.wait(),
                anim_ticker.next(),
            )
            .await;

            let mut state_changed = false;

            match select_res {
                Either3::First(mes) => match mes {
                    DisplayMessage::Clear => {
                        let _ = display.clear().await;
                        state_changed = true;
                    }
                    DisplayMessage::Message(_msg) => {
                        // Optional custom message rendering
                    }
                    DisplayMessage::Output(output) => {
                        output_mode = output;
                        state_changed = true;
                    }
                    DisplayMessage::LayerState(layers) => {
                        layer_state = layers;
                        state_changed = true;
                    }
                    DisplayMessage::MouseAvailable(mouse) => {
                        mouse_available = mouse;
                        state_changed = true;
                    }
                    DisplayMessage::NumLock(nl) => {
                        num_lock = nl;
                        state_changed = true;
                    }
                    DisplayMessage::CapsLock(cl) => {
                        caps_lock = cl;
                        state_changed = true;
                    }
                    DisplayMessage::Brightness(brightness) => {
                        let _ = display.set_brightness(brightness).await;
                    }
                    DisplayMessage::On(on) => {
                        let _ = display.set_display_on(on).await;
                    }
                    _ => {}
                },
                Either3::Second(_str) => {
                    // Optional dynamic message string rendering
                }
                Either3::Third(_) => {
                    // Idle animation timer tick
                    anim_tick = anim_tick.wrapping_add(1);
                    state_changed = true;
                }
            }

            if state_changed {
                draw_dashboard(
                    display,
                    &layer_state,
                    caps_lock,
                    num_lock,
                    output_mode,
                    mouse_available,
                    anim_tick,
                )
                .await;
                let _ = display.flush().await;
            }
        }
    }
}
