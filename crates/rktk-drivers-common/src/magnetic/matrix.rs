use rktk::drivers::interface::keyscan::{KeyChangeEvent, KeyscanDriver};
use rktk::drivers::interface::magnetic::{Adc, Multiplexer};
use crate::magnetic::rapid_trigger::RapidTriggerState;

pub trait MagneticScanner {
    type Error: core::fmt::Debug;
    async fn scan(&mut self, row: usize, col: usize) -> Result<u16, Self::Error>;
}

pub struct DirectScanner<A: Adc, const ROWS: usize, const COLS: usize> {
    adcs: [[A; COLS]; ROWS],
}

impl<A: Adc, const ROWS: usize, const COLS: usize> DirectScanner<A, ROWS, COLS> {
    pub fn new(adcs: [[A; COLS]; ROWS]) -> Self {
        Self { adcs }
    }
}

impl<A: Adc, const ROWS: usize, const COLS: usize> MagneticScanner for DirectScanner<A, ROWS, COLS> {
    type Error = A::Error;
    async fn scan(&mut self, row: usize, col: usize) -> Result<u16, Self::Error> {
        self.adcs[row][col].read().await
    }
}

pub struct MuxScanner<A: Adc, M: Multiplexer, F: Fn(usize, usize) -> (u8, u8)> {
    adc: A,
    mux: M,
    map: F,
}

impl<A: Adc, M: Multiplexer, F: Fn(usize, usize) -> (u8, u8)> MuxScanner<A, M, F> {
    pub fn new(adc: A, mux: M, map: F) -> Self {
        Self { adc, mux, map }
    }
}

#[derive(Debug)]
pub enum MagneticError<AE, ME> {
    Adc(AE),
    Mux(ME),
}

impl<A: Adc, M: Multiplexer, F: Fn(usize, usize) -> (u8, u8)> MagneticScanner for MuxScanner<A, M, F> {
    type Error = MagneticError<A::Error, M::Error>;
    async fn scan(&mut self, row: usize, col: usize) -> Result<u16, Self::Error> {
        let (mux_ch, _adc_ch) = (self.map)(row, col);
        self.mux.select(mux_ch).await.map_err(MagneticError::Mux)?;
        self.adc.read().await.map_err(MagneticError::Adc)
    }
}

pub struct MagneticMatrix<S: MagneticScanner, const ROWS: usize, const COLS: usize> {
    scanner: S,
    states: [[RapidTriggerState; COLS]; ROWS],
    press_dist: u16,
    release_dist: u16,
}

impl<S: MagneticScanner, const ROWS: usize, const COLS: usize> MagneticMatrix<S, ROWS, COLS> {
    pub fn new(scanner: S, press_dist: u16, release_dist: u16) -> Self {
        Self {
            scanner,
            states: [[RapidTriggerState::new(); COLS]; ROWS],
            press_dist,
            release_dist,
        }
    }
}

impl<S: MagneticScanner, const ROWS: usize, const COLS: usize> KeyscanDriver for MagneticMatrix<S, ROWS, COLS> {
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        for row in 0..ROWS {
            for col in 0..COLS {
                match self.scanner.scan(row, col).await {
                    Ok(val) => {
                        if let Some(pressed) = self.states[row][col].update(val, self.press_dist, self.release_dist) {
                            cb(KeyChangeEvent {
                                row: row as u8,
                                col: col as u8,
                                pressed,
                            });
                        }
                    }
                    Err(e) => {
                        rktk_log::error!("Magnetic scan error at {},{}: {:?}", row, col, e);
                    }
                }
            }
        }
    }
}
