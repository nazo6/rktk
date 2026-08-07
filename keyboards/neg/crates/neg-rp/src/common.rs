pub fn init_peri() -> Peripherals {
    let p = embassy_rp::init(Default::default());

    #[cfg(feature = "alloc")]
    unsafe {
        use crate::HEAP;
        embedded_alloc::init!(HEAP, 32768);
    }

    p
}

use embassy_rp::Peripherals;
