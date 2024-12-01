use rktk_keymanager::state::{EncoderDirection, KeyChangeEvent, StateReport};

use crate::drivers::interface::{keyscan::KeyscanDriver, mouse::MouseDriver};

/// Hooks called for master side
pub trait MasterHooks {
    /// Called after master side initialization.
    async fn on_master_init(
        &mut self,
        _key_scanner: &mut impl KeyscanDriver,
        _mouse: Option<&mut impl MouseDriver>,
        // _reporter: &impl ReporterDriver,
    ) {
    }

    /// Called after keyboard event occur, before state update and report send.
    ///
    /// # Parameters
    /// - `events`: The keyboard events.
    ///
    /// # Returns
    /// If false, this event will be ignored.
    async fn on_keyboard_event(&mut self, _event: &mut KeyChangeEvent) -> bool {
        true
    }

    /// Called after mouse move event occur, before state update and report send.
    ///
    /// # Parameters
    /// - `mouse_move`: The mouse move event. If None, no mouse move event occurred. If Some, the
    ///   tuple contains the x and y offset of the mouse move event.
    ///
    /// # Returns
    /// If false, this event will be ignored.
    async fn on_mouse_event(&mut self, _mouse_move: &mut (i8, i8)) -> bool {
        true
    }

    /// Called after encoder event occur, before state update and report send.
    ///
    /// # Parameters
    /// - `id`: The encoder id.
    /// - `dir`: The encoder direction.
    ///
    /// # Returns
    /// If false, this event will be ignored.
    async fn on_encoder_event(&mut self, _id: &mut u8, _dir: &mut EncoderDirection) -> bool {
        true
    }

    /// Called after state update, before report send.
    ///
    /// WARNING: Mutating the state_report or returning false is not recommended as this can cause
    /// inconsistent state.
    ///
    /// # Parameters
    /// - `state_report`: Report returned from rktk-keymanager's update function.
    ///
    /// # Returns
    /// If false, this report will be ignored.
    async fn on_state_update(&mut self, _state_report: &mut StateReport) -> bool {
        true
    }
}
