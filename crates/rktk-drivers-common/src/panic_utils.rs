//! Panic message handling utilities.
//!
//! Because doing something complicated in panic_handler is difficult (eg. async function can't be called),
//! alternatively you can reboot the device in panic_handler.
//! By saving panic info in uninit section, you can display panic message on display after reboot.
//!
//! Note that this module depends on .uninit section, which is handled by cortex_m_rt.

use core::{fmt::Write, mem::MaybeUninit, ptr::write_volatile};

use embassy_time::Timer;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, ascii::FONT_8X13},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use rktk::{
    drivers::interface::display::DisplayDriver,
    task::display::utils::{RotatedDrawTarget, Rotation},
};

#[unsafe(link_section = ".uninit.PANICINFO")]
static mut PANIC_INFO: MaybeUninit<PanicMessage> = MaybeUninit::uninit();
const PANIC_INFO_MAGIC: u32 = 0x54_41_43_4B;

#[repr(C)]
pub struct PanicMessage {
    magic: u32,
    len: u16,
    data: [u8; 1024],
}

impl Write for PanicMessage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let writable_str = if s.len() + self.len as usize > self.data.len() {
            let writable_len = self.data.len() - self.len as usize;
            if writable_len == 0 {
                return Ok(());
            }
            s.get(..writable_len).unwrap_or(s)
        } else {
            s
        };

        let new_len = writable_str.len() as u16 + self.len;
        self.data[self.len as usize..new_len as usize].copy_from_slice(writable_str.as_bytes());
        self.len = new_len;

        Ok(())
    }
}

impl Default for PanicMessage {
    fn default() -> Self {
        Self {
            magic: PANIC_INFO_MAGIC,
            len: 0,
            data: [0; 1024],
        }
    }
}

impl PanicMessage {
    fn reset() -> Self {
        Self {
            magic: 0,
            len: 0,
            data: [0; 1024],
        }
    }
}

/// Save panic info to uninit section.
///
/// This function should be called in panic_handler.
pub fn save_panic_info(info: &core::panic::PanicInfo) {
    let mut panic_info = PanicMessage::default();
    write!(panic_info, "{info}").ok();

    unsafe {
        write_volatile(&raw mut PANIC_INFO, MaybeUninit::new(panic_info));
    }
}

fn read_panic_message() -> Option<PanicMessage> {
    unsafe {
        let info = core::ptr::read(&raw const PANIC_INFO);
        let info = info.assume_init();
        if info.magic == PANIC_INFO_MAGIC {
            write_volatile(&raw mut PANIC_INFO, MaybeUninit::new(PanicMessage::reset()));
            Some(info)
        } else {
            None
        }
    }
}

fn parse_panic_message(panic_info: &PanicMessage) -> &str {
    // if panic_info.len == 0 {
    //     return "No panic message";
    // }
    match core::str::from_utf8(&panic_info.data[..panic_info.len as usize]) {
        Ok(str) => str,
        Err(e) => {
            let valid_len = e.valid_up_to();
            core::str::from_utf8(&panic_info.data[..valid_len]).unwrap()
        }
    }
}

const FONT: MonoFont = FONT_8X13;

/// Display panic message on display is exists.
/// This is intended to be called before [`rktk::task::start`] and takes a display builder.
///
/// If previous panic message exists, this function will display it on the display and return None.
/// Otherwise, it will return the display builder.
///
/// When None is returned, caller can stop execution using something like [`cortex_m::asm::udf`]
pub async fn display_message_if_panicked<D: DisplayDriver>(display: &mut D) {
    if let Some(panic_info) = read_panic_message()
        && display.init().await.is_ok() {
            let char_width = FONT.character_size.width as usize;

            let str = parse_panic_message(&panic_info);

            rktk_log::error!("Previous panic detected: {:?}", str);

            let str_len = str
                .lines()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0) as usize;

            let orig_display_size = display.as_mut().bounding_box().size;
            let rotation = if orig_display_size.width > orig_display_size.height {
                Rotation::Rotate0
            } else {
                Rotation::Rotate90
            };

            let rotated_display = RotatedDrawTarget::new(display.as_mut(), rotation);
            let display_width = rotated_display.bounding_box().size.width as usize;
            let overflow_len = if str_len * char_width > display_width {
                str_len - display_width / char_width + 1
            } else {
                0
            };

            display_mes(display, "Panic!", Point::zero(), rotation).await;
            Timer::after_millis(400).await;

            if overflow_len > 0 {
                loop {
                    for i in 0..=overflow_len {
                        display_mes(
                            display,
                            str,
                            Point::new(-((i * char_width) as i32), 0),
                            rotation,
                        )
                        .await;
                        Timer::after_millis(200).await;
                    }
                }
            } else {
                display_mes(display, str, Point::zero(), rotation).await;
                Timer::after_secs(100000000).await;
            }
        }
}
pub async fn display_mes<D: DisplayDriver>(
    display: &mut D,
    str: &str,
    pos: Point,
    rotation: Rotation,
) {
    {
        let mut rotated_display = RotatedDrawTarget::new(display.as_mut(), rotation);

        let _ = rotated_display.clear(BinaryColor::Off);
        let _ = Text::with_baseline(
            str,
            pos,
            MonoTextStyle::new(&FONT, BinaryColor::On),
            Baseline::Top,
        )
        .draw(&mut rotated_display);
    }
    let _ = display.flush().await;
}
