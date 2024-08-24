//! Panic message handling
//!
//! do something complicated in panic_handler is difficult (eg. async function can't be called).
//! So, if panic occurs, save panic message to uninitialized memory and read it after reboot.
//!
//! By using this method, you can display panic message on display after reboot.
//!
//! Note that this module depends on .uninit section, which is handled by cortex_m_rt.

use core::{
    fmt::Write,
    mem::MaybeUninit,
    ptr::{read_volatile, write_volatile},
};

pub struct PanicMessage {
    magic: u32,
    data: heapless::Vec<u8, 1008>,
}

impl Write for PanicMessage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.data
            .extend_from_slice(s.as_bytes())
            .map_err(|_| core::fmt::Error)?;
        Ok(())
    }
}

#[link_section = ".uninit.PANICINFO"]
static mut PANIC_INFO: MaybeUninit<PanicMessage> = MaybeUninit::uninit();
const PANIC_INFO_MAGIC: [u8; 4] = [0x54, 0x41, 0x43, 0x4B];

pub fn save_panic_info(info: &core::panic::PanicInfo) {
    let mut panic_info = PanicMessage {
        magic: u32::from_le_bytes(PANIC_INFO_MAGIC),
        data: heapless::Vec::new(),
    };
    write!(panic_info, "{}", info).ok();

    unsafe {
        write_volatile(PANIC_INFO.as_mut_ptr(), panic_info);
    }
}

fn read_panic_message() -> Option<PanicMessage> {
    unsafe {
        let panic_info = read_volatile(PANIC_INFO.as_ptr());
        if panic_info.magic == u32::from_le_bytes(PANIC_INFO_MAGIC) {
            write_volatile(
                PANIC_INFO.as_mut_ptr(),
                PanicMessage {
                    magic: 0,
                    data: heapless::Vec::new(),
                },
            );
            Some(panic_info)
        } else {
            None
        }
    }
}

fn parse_panic_message(panic_info: &PanicMessage) -> &str {
    let str = match core::str::from_utf8(&panic_info.data) {
        Ok(str) => str,
        Err(e) => {
            let valid_len = e.valid_up_to();
            core::str::from_utf8(&panic_info.data[..valid_len]).unwrap()
        }
    };
    str
}

/// Display panic message on display is exists.
/// This is intended to be called before [`rktk::task::start`] and takes a display builder.
///
/// If previous panic message exists, this function will display it on the display and return None.
/// Otherwise, it will return the display builder.
///
/// When None is returned, caller can stop execution using something like [`cortex_m::asm::udf`]
pub async fn display_message_if_panicked<
    D: rktk::interface::display::DisplayDriver,
    DB: rktk::interface::DriverBuilder<Output = D>,
>(
    display_builder: DB,
) -> Option<DB> {
    if let Some(panic_info) = read_panic_message() {
        if let Ok(mut display) = display_builder.build().await {
            let str = parse_panic_message(&panic_info);

            if str.len() > D::MAX_TEXT_WIDTH {
                let mut idx = 0;
                loop {
                    let _ = display
                        .update_text(
                            &str[idx..],
                            embedded_graphics::prelude::Point { x: 0, y: 0 },
                        )
                        .await;
                    if str.len() - idx <= D::MAX_TEXT_WIDTH {
                        embassy_time::Timer::after_millis(600).await;
                        idx = 0;
                    } else {
                        idx += 1;
                    }
                    embassy_time::Timer::after_millis(200).await;
                }
            } else {
                let _ = display.update_text(str, D::calculate_point(1, 1)).await;
            }
        }
        None
    } else {
        Some(display_builder)
    }
}
