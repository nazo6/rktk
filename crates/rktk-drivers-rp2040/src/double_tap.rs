use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    ptr::{read_volatile, write_volatile},
};

use embassy_time::{Duration, Timer};
use rktk::interface::double_tap::DoubleTapReset;

const BOOTLOADER_MAGIC: u32 = 0xDEADBEEF;

#[link_section = ".uninit.FLAG"]
static mut FLAG: UnsafeCell<MaybeUninit<u32>> = UnsafeCell::new(MaybeUninit::uninit());

pub struct DoubleTapResetRp;

impl DoubleTapReset for DoubleTapResetRp {
    async fn execute(&self, timeout: Duration) {
        unsafe {
            if read_volatile(FLAG.get().cast::<u32>()) == BOOTLOADER_MAGIC {
                write_volatile(FLAG.get().cast(), 0);

                embassy_rp::rom_data::reset_to_usb_boot(0, 0);
            }

            write_volatile(FLAG.get().cast(), BOOTLOADER_MAGIC);

            Timer::after(timeout).await;

            write_volatile(FLAG.get().cast(), 0);
        }
    }
}
