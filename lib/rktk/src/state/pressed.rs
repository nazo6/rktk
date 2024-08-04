use embassy_time::{Duration, Instant};

use crate::{config::static_config::CONFIG, interface::keyscan::KeyChangeEventOneHand};

pub struct AllPressed {
    state: [[Option<Instant>; CONFIG.cols * 2]; CONFIG.rows],
}

pub struct KeyStatusUpdateEvent {
    pub row: u8,
    pub col: u8,
    pub change_type: KeyStatusChangeType,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyStatusChangeType {
    // Just pressed
    Pressed,
    // Still pressing
    Pressing(Duration),
    // Released
    Released(Duration),
}
impl AllPressed {
    pub fn new() -> Self {
        Self {
            state: [[None; CONFIG.cols * 2]; CONFIG.rows],
        }
    }
    pub fn compose_events_and_update_pressed<'a>(
        &mut self,
        events: impl Iterator<Item = &'a KeyChangeEventOneHand>,
        update_time: Instant,
    ) -> heapless::Vec<KeyStatusUpdateEvent, 32> {
        let mut composed = heapless::Vec::new();
        for event in events {
            if event.row as usize >= CONFIG.rows || event.col as usize >= (CONFIG.cols * 2) {
                continue;
            }
            let key_state = &mut self.state[event.row as usize][event.col as usize];
            match (event.pressed, &key_state) {
                (true, None) => {
                    *key_state = Some(update_time);
                    composed
                        .push(KeyStatusUpdateEvent {
                            row: event.row,
                            col: event.col,
                            change_type: KeyStatusChangeType::Pressed,
                        })
                        .ok();
                }
                (false, Some(pressed_time)) => {
                    composed
                        .push(KeyStatusUpdateEvent {
                            row: event.row,
                            col: event.col,
                            change_type: KeyStatusChangeType::Released(update_time - *pressed_time),
                        })
                        .ok();
                    *key_state = None;
                }
                _ => {}
            }
        }

        for (row, col, press_start) in self.iter() {
            if !composed.iter().any(|e| e.row == row && e.col == col) {
                composed
                    .push(KeyStatusUpdateEvent {
                        row,
                        col,
                        change_type: KeyStatusChangeType::Pressing(update_time - *press_start),
                    })
                    .ok();
            }
        }

        composed
    }

    pub fn iter(&self) -> PressedIter {
        PressedIter {
            pressed: self,
            idx_row: 0,
            idx_col: 0,
        }
    }
}

pub struct PressedIter<'a> {
    pressed: &'a AllPressed,
    idx_row: usize,
    idx_col: usize,
}

impl<'a> Iterator for PressedIter<'a> {
    type Item = (u8, u8, &'a Instant);
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.idx_row..(CONFIG.rows) {
            for j in self.idx_col..(CONFIG.cols * 2) {
                let key_state = &self.pressed.state[i][j];
                if let Some(press_start) = key_state {
                    self.idx_row = i;
                    self.idx_col = j + 1;

                    let row = i as u8;
                    let col = j as u8;

                    return Some((row, col, press_start));
                }
            }
            self.idx_col = 0;
        }
        None
    }
}
