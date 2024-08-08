use heapless::Vec;

use crate::{
    config::static_config::CONFIG,
    interface::keyscan::{Hand, KeyChangeEventOneHand},
};

pub struct Pressed([[bool; CONFIG.cols * 2]; CONFIG.rows]);

pub struct KeyStatusEvents {
    pub pressed: Vec<KeyLocation, 16>,
    pub pressing: Vec<KeyLocation, 32>,
    pub released: Vec<KeyLocation, 16>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct KeyLocation {
    pub row: u8,
    pub col: u8,
}

impl Pressed {
    pub fn new() -> Self {
        Self([[false; CONFIG.cols * 2]; CONFIG.rows])
    }

    /// Compose one-hand events to entire keyboard events and update keys status with the events
    pub fn compose_events_and_update_pressed(
        &mut self,
        master_hand: Option<Hand>,
        master_events: &mut [KeyChangeEventOneHand],
        slave_events: &mut [KeyChangeEventOneHand],
    ) -> KeyStatusEvents {
        let events = {
            let (left_events, right_events) = if master_hand == Some(Hand::Right) {
                (slave_events, master_events)
            } else {
                // If the keyboard is not split, the master hand is the left hand (zero-index)
                (master_events, slave_events)
            };
            right_events.iter_mut().for_each(|event| {
                event.col = ((CONFIG.cols - 1) as u8 - event.col) + CONFIG.cols as u8;
            });

            right_events.iter().chain(left_events.iter())
        };

        let mut pressed_events = Vec::new();
        let mut released_events = Vec::new();
        for event in events {
            if event.row as usize >= CONFIG.rows || event.col as usize >= (CONFIG.cols * 2) {
                // invalid event
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
