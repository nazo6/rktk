use embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice;
use embassy_nrf::{
    gpio::{Input, Output, Pin},
    spim::{Instance, Spim},
    Peripheral,
};
use embassy_sync::{blocking_mutex::raw::RawMutex, mutex::Mutex};
use rktk_drivers_common::keyscan::shift_register_matrix::ShiftRegisterMatrix;

pub fn create_shift_register_matrix<
    'a,
    'd: 'a,
    M: RawMutex,
    T: Instance + 'd,
    CS: Peripheral<P = impl Pin> + 'd,
    const OUTPUT_PIN_COUNT: usize,
    const INPUT_PIN_COUNT: usize,
    const COLS: usize,
    const ROWS: usize,
>(
    shared_spi: &'a Mutex<M, Spim<'d, T>>,
    ncs: CS,
    input_pins: [Input<'d>; INPUT_PIN_COUNT],
    left_detect_key: (usize, usize),
    map_key: fn(usize, usize) -> Option<(usize, usize)>,
) -> ShiftRegisterMatrix<
    SpiDevice<'a, M, Spim<'d, T>, Output<'d>>,
    Input<'d>,
    OUTPUT_PIN_COUNT,
    INPUT_PIN_COUNT,
    COLS,
    ROWS,
> {
    let cs_output = Output::new(
        ncs,
        embassy_nrf::gpio::Level::High,
        embassy_nrf::gpio::OutputDrive::Standard,
    );
    let spi_device = SpiDevice::new(shared_spi, cs_output);

    ShiftRegisterMatrix::new(spi_device, input_pins, left_detect_key, map_key)
}
