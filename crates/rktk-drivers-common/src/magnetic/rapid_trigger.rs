#[derive(Debug, Clone, Copy)]
pub struct RapidTriggerState {
    last_val: u16,
    max_val: u16,
    min_val: u16,
    pressed: bool,
}

impl RapidTriggerState {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            last_val: 0,
            max_val: 0,
            min_val: 0,
            pressed: false,
        }
    }

    /// Updates the state with a new ADC value and returns whether the key state changed.
    ///
    /// # Arguments
    /// * `val`: Normalized ADC value (0-65535).
    /// * `press_dist`: Threshold to trigger press when moving down.
    /// * `release_dist`: Threshold to trigger release when moving up.
    pub fn update(&mut self, val: u16, press_dist: u16, release_dist: u16) -> Option<bool> {
        let changed = if !self.pressed {
            if val > self.min_val.saturating_add(press_dist) {
                self.pressed = true;
                self.max_val = val;
                Some(true)
            } else {
                if val < self.min_val {
                    self.min_val = val;
                }
                None
            }
        } else {
            if val < self.max_val.saturating_sub(release_dist) {
                self.pressed = false;
                self.min_val = val;
                Some(false)
            } else {
                if val > self.max_val {
                    self.max_val = val;
                }
                None
            }
        };

        self.last_val = val;
        changed
    }
}
