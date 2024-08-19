use heapless::Vec;

use super::KeyChangeEvent;

pub struct Pressed<const COL: usize, const ROW: usize>([[bool; COL]; ROW]);

pub struct KeyStatusEvents {
    pub pressed: Vec<KeyLocation, 16>,
    pub pressing: Vec<KeyLocation, 32>,
    pub released: Vec<KeyLocation, 16>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct KeyLocation {
    pub row: u8,
    pub col: u8,
}

impl<const COL: usize, const ROW: usize> Pressed<COL, ROW> {
    pub fn new() -> Self {
        Self([[false; COL]; ROW])
    }

    /// Compose one-hand events to entire keyboard events and update keys status with the events
    pub fn update_pressed(&mut self, events: &mut [KeyChangeEvent]) -> KeyStatusEvents {
        let mut pressed_events = Vec::new();
        let mut released_events = Vec::new();
        for event in events {
            if event.row as usize >= ROW || event.col as usize >= COL {
                continue;
            }
            let key_pressed = &mut self.0[event.row as usize][event.col as usize];
            match (event.pressed, *key_pressed) {
                (true, false) => {
                    *key_pressed = true;
                    pressed_events
                        .push(KeyLocation {
                            row: event.row,
                            col: event.col,
                        })
                        .ok();
                }
                (false, true) => {
                    *key_pressed = false;
                    released_events
                        .push(KeyLocation {
                            row: event.row,
                            col: event.col,
                        })
                        .ok();
                }
                _ => {}
            }
        }

        let mut pressing_events = Vec::new();

        for (row, row_arr) in self.0.iter().enumerate() {
            for (col, pressed) in row_arr.iter().enumerate() {
                let col = col as u8;
                let row = row as u8;
                if *pressed
                    && !pressed_events
                        .iter()
                        .chain(released_events.iter())
                        .any(|e| e.row == row && e.col == col)
                {
                    pressing_events.push(KeyLocation { row, col }).ok();
                }
            }
        }

        KeyStatusEvents {
            pressed: pressed_events,
            pressing: pressing_events,
            released: released_events,
        }
    }
}
