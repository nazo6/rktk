use core::fmt::{self, Formatter};

use rktk_log::warn;

pub struct Pressed<const ROWS: usize, const COLS: usize>([[bool; COLS]; ROWS]);

impl<const ROWS: usize, const COLS: usize> Pressed<ROWS, COLS> {
    pub fn new() -> Self {
        Self([[false; COLS]; ROWS])
    }
    /// Returns None if the key state is not changed.
    /// Returns Some(true) if the key is changed to pressed.
    /// Returns Some(false) if the key is changed to released.
    pub fn set_pressed(&mut self, pressed: bool, row: usize, col: usize) -> Option<bool> {
        let Some(Some(prev_pressed)) = self.0.get_mut(row).map(|r| r.get_mut(col)) else {
            warn!("Invalid key position: row={}, col={}", row, col);
            return None;
        };

        if prev_pressed == &pressed {
            None
        } else if !*prev_pressed {
            *prev_pressed = true;
            Some(true)
        } else {
            *prev_pressed = false;
            Some(false)
        }
    }
}

impl<const ROWS: usize, const COLS: usize> core::fmt::Debug for Pressed<ROWS, COLS> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (row, col) in self.iter() {
            write!(f, "{row},{col} ")?;
        }
        Ok(())
    }
}

pub struct PressedIter<'a, const ROWS: usize, const COLS: usize> {
    pressed: &'a Pressed<ROWS, COLS>,
    idx_row: usize,
    idx_col: usize,
}

impl<const ROWS: usize, const COLS: usize> Iterator for PressedIter<'_, ROWS, COLS> {
    type Item = (u8, u8);
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx_row..ROWS {
            for j in self.idx_col..COLS {
                let pressed = &self.pressed.0[i][j];
                if *pressed {
                    self.idx_row = i;
                    self.idx_col = j + 1;

                    let row = i as u8;
                    let col = j as u8;

                    return Some((row, col));
                }
            }
            self.idx_col = 0;
        }
        None
    }
}

impl<const ROWS: usize, const COLS: usize> Pressed<ROWS, COLS> {
    pub fn iter(&self) -> PressedIter<ROWS, COLS> {
        PressedIter {
            pressed: self,
            idx_row: 0,
            idx_col: 0,
        }
    }
}
