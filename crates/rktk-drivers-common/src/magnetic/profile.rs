pub trait SwitchProfile {
    /// Max travel distance of the switch in 0.01mm (e.g. 400 for 4.0mm)
    fn max_travel(&self) -> u16;

    /// Maps normalized value (0-65535) to distance (0 to max_travel)
    fn normalized_to_distance(&self, normalized: u16) -> u16;
}

/// Simple linear switch profile.
pub struct LinearProfile {
    pub max_travel: u16,
}

impl SwitchProfile for LinearProfile {
    fn max_travel(&self) -> u16 {
        self.max_travel
    }

    fn normalized_to_distance(&self, normalized: u16) -> u16 {
        ((normalized as u32 * self.max_travel as u32) / 65535) as u16
    }
}

/// High-performance 17-point lookup table profile.
///
/// Divides the 0-65535 range into exactly 16 intervals of 4096 wide.
/// Uses fast bit-shifting for interpolation without divisions.
pub struct Lut17Profile {
    pub max_travel: u16,
    pub lut: [u16; 17],
}

impl SwitchProfile for Lut17Profile {
    fn max_travel(&self) -> u16 {
        self.max_travel
    }

    fn normalized_to_distance(&self, normalized: u16) -> u16 {
        let idx = (normalized >> 12) as usize;
        if idx >= 16 {
            return self.lut[16];
        }
        let y0 = self.lut[idx] as u32;
        let y1 = self.lut[idx + 1] as u32;
        let fraction = (normalized & 0xFFF) as u32; // 0..4095
        let val = y0 + (((y1 as i32 - y0 as i32) * fraction as i32) >> 12) as u32;
        val as u16
    }
}

pub trait KeyProfileMap<const ROWS: usize, const COLS: usize> {
    type Profile: SwitchProfile;
    fn get_profile(&self, row: usize, col: usize) -> &Self::Profile;
}

pub struct SingleProfileMap<P: SwitchProfile> {
    pub profile: P,
}

impl<P: SwitchProfile, const ROWS: usize, const COLS: usize> KeyProfileMap<ROWS, COLS> for SingleProfileMap<P> {
    type Profile = P;
    fn get_profile(&self, _row: usize, _col: usize) -> &Self::Profile {
        &self.profile
    }
}

pub struct MatrixProfileMap<P: SwitchProfile, const ROWS: usize, const COLS: usize> {
    pub profiles: [[P; COLS]; ROWS],
}

impl<P: SwitchProfile, const ROWS: usize, const COLS: usize> KeyProfileMap<ROWS, COLS> for MatrixProfileMap<P, ROWS, COLS> {
    type Profile = P;
    fn get_profile(&self, row: usize, col: usize) -> &Self::Profile {
        &self.profiles[row][col]
    }
}
