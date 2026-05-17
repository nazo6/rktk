use crate::magnetic::rapid_trigger::RapidTriggerState;
use crate::magnetic::profile::{KeyProfileMap, SwitchProfile};
use rktk::drivers::interface::keyscan::{KeyChangeEvent, KeyscanDriver};
use rktk::drivers::interface::magnetic::{Adc, Multiplexer};
use rktk_log::MaybeFormat;

pub trait MagneticScanner {
    type Error: core::fmt::Debug + MaybeFormat;
    fn scan(
        &mut self,
        row: usize,
        col: usize,
    ) -> impl core::future::Future<Output = Result<u16, Self::Error>>;
}

pub struct DirectScanner<A: Adc, const ROWS: usize, const COLS: usize> {
    adcs: [[A; COLS]; ROWS],
}

impl<A: Adc, const ROWS: usize, const COLS: usize> DirectScanner<A, ROWS, COLS> {
    pub fn new(adcs: [[A; COLS]; ROWS]) -> Self {
        Self { adcs }
    }
}

impl<A: Adc, const ROWS: usize, const COLS: usize> MagneticScanner
    for DirectScanner<A, ROWS, COLS>
{
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
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MagneticError<AE, ME> {
    Adc(AE),
    Mux(ME),
}

impl<A: Adc, M: Multiplexer, F: Fn(usize, usize) -> (u8, u8)> MagneticScanner
    for MuxScanner<A, M, F>
{
    type Error = MagneticError<A::Error, M::Error>;
    async fn scan(&mut self, row: usize, col: usize) -> Result<u16, Self::Error> {
        let (mux_ch, _adc_ch) = (self.map)(row, col);
        self.mux.select(mux_ch).await.map_err(MagneticError::Mux)?;
        self.adc.read().await.map_err(MagneticError::Adc)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CalibrationEntry {
    pub min: u16,
    pub max: u16,
}

impl CalibrationEntry {
    pub const fn new() -> Self {
        Self {
            min: u16::MAX,
            max: u16::MIN,
        }
    }
}

impl Default for CalibrationEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MagneticMatrix<S: MagneticScanner, M: KeyProfileMap<ROWS, COLS>, const ROWS: usize, const COLS: usize> {
    scanner: S,
    profiles: M,
    states: [[RapidTriggerState; COLS]; ROWS],
    calibration_mode: bool,
    calibration_data: [[CalibrationEntry; COLS]; ROWS],
    press_dist: u16,
    release_dist: u16,
}

impl<S: MagneticScanner, M: KeyProfileMap<ROWS, COLS>, const ROWS: usize, const COLS: usize> MagneticMatrix<S, M, ROWS, COLS> {
    pub fn new(scanner: S, profiles: M, press_dist: u16, release_dist: u16) -> Self {
        Self {
            scanner,
            profiles,
            states: [[RapidTriggerState::new(); COLS]; ROWS],
            calibration_mode: false,
            calibration_data: [[CalibrationEntry::new(); COLS]; ROWS],
            press_dist,
            release_dist,
        }
    }

    pub fn set_calibration_mode(&mut self, enabled: bool) {
        self.calibration_mode = enabled;
        if enabled {
            // Reset calibration data when starting
            self.calibration_data = [[CalibrationEntry::new(); COLS]; ROWS];
        }
    }

    pub fn get_calibration_data(&self) -> [[CalibrationEntry; COLS]; ROWS] {
        self.calibration_data
    }

    pub fn set_calibration_data(&mut self, data: [[CalibrationEntry; COLS]; ROWS]) {
        self.calibration_data = data;
    }
}

impl<S: MagneticScanner, M: KeyProfileMap<ROWS, COLS>, const ROWS: usize, const COLS: usize> KeyscanDriver
    for MagneticMatrix<S, M, ROWS, COLS>
{
    async fn scan(&mut self, mut cb: impl FnMut(KeyChangeEvent)) {
        for row in 0..ROWS {
            for col in 0..COLS {
                match self.scanner.scan(row, col).await {
                    Ok(val) => {
                        if self.calibration_mode {
                            let entry = &mut self.calibration_data[row][col];
                            if val < entry.min {
                                entry.min = val;
                            }
                            if val > entry.max {
                                entry.max = val;
                            }
                        } else {
                            let entry = &self.calibration_data[row][col];
                            if entry.min < entry.max {
                                // Normalize value to 0-65535
                                let normalized = if val <= entry.min {
                                    0
                                } else if val >= entry.max {
                                    65535
                                } else {
                                    ((val - entry.min) as u32 * 65535
                                        / (entry.max - entry.min) as u32)
                                        as u16
                                };

                                let profile = self.profiles.get_profile(row, col);
                                let distance = profile.normalized_to_distance(normalized);

                                if let Some(pressed) = self.states[row][col].update(
                                    distance,
                                    self.press_dist,
                                    self.release_dist,
                                ) {
                                    cb(KeyChangeEvent {
                                        row: row as u8,
                                        col: col as u8,
                                        pressed,
                                    });
                                }
                            }
                        }
                    }
                    Err(e) => {
                        rktk_log::error!("Magnetic scan error at {},{}: {:?}", row, col, e);
                    }
                }
            }
        }
    }

    fn start_calibration(&mut self) {
        self.set_calibration_mode(true);
        rktk_log::info!("Calibration started. Press all keys to their physical limits.");
    }

    fn end_calibration(&mut self) {
        self.set_calibration_mode(false);
        rktk_log::info!("Calibration finished. Results:");
        for row in 0..ROWS {
            for col in 0..COLS {
                let entry = self.calibration_data[row][col];
                if entry.min < entry.max {
                    rktk_log::info!(
                        "  Key ({},{}): min_adc={}, max_adc={}, range={}",
                        row,
                        col,
                        entry.min,
                        entry.max,
                        entry.max - entry.min
                    );
                } else {
                    rktk_log::info!("  Key ({},{}): Not calibrated", row, col);
                }
            }
        }
    }
}
