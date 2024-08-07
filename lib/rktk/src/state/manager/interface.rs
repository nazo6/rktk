use crate::{
    keycode::KeyCode,
    state::common::{CommonLocalState, CommonState},
};

use super::super::pressed::KeyStatusEvent;

pub(crate) trait LocalStateManager {
    type GlobalState;
    type Report;

    /// Called for every key events
    fn process_event(
        &mut self,
        common_state: &mut CommonState,
        common_local_state: &mut CommonLocalState,
        global_state: &mut Self::GlobalState,
        kc: &KeyCode,
        event: &KeyStatusEvent,
    );

    /// Called once for every loop
    fn loop_end(
        &mut self,
        _common_state: &mut CommonState,
        _common_local_state: &mut CommonLocalState,
        _global_state: &mut Self::GlobalState,
    ) {
    }

    /// Called once for every loop and should return a report.
    fn report(
        self,
        common_state: &CommonState,
        common_local_state: &CommonLocalState,
        global_state: &mut Self::GlobalState,
    ) -> Option<Self::Report>;
}
