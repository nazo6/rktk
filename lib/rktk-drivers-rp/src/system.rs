use core::{mem::MaybeUninit, ptr::write_volatile};

use embassy_time::{Duration, Timer};
use rktk::drivers::interface::system::SystemDriver;

const BOOTLOADER_MAGIC: u32 = 0xABCD_EF01;

#[link_section = ".uninit"]
static mut FLAG: MaybeUninit<u32> = MaybeUninit::uninit();

pub struct RpSystemDriver;

impl SystemDriver for RpSystemDriver {
    async fn double_reset_usb_boot(&self, timeout: Duration) {
        unsafe {
            let flag = core::ptr::read(&raw const FLAG);
            let flag = flag.assume_init();

            if flag == BOOTLOADER_MAGIC {
                // double reset is triggered. rebooting to bootloader.
                write_volatile(&raw mut FLAG, MaybeUninit::new(0));
                embassy_rp::rom_data::reset_to_usb_boot(0, 0);
            }

            // write flag and wait for double reset
            write_volatile(&raw mut FLAG, MaybeUninit::new(BOOTLOADER_MAGIC));
            Timer::after(timeout).await;
            // double-tap reset is not performed. reset flag and normal start
            write_volatile(&raw mut FLAG, MaybeUninit::new(0));
        }
    }

    fn reset(&self) {
        // not supported
    }

    fn reset_to_bootloader(&self) {
        embassy_rp::rom_data::reset_to_usb_boot(0, 0);
    }
}
