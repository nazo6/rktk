pub use rktk_keymanager::interface::state::input_event::KeyChangeEvent;

/// Key scanner driver interface.
///
/// The keyscan driver has two roles:
/// - Scanning the keys
/// - Determining which hand is currently using the keyboard on a split keyboard
///
/// This is because the key scanning circuit often includes a left/right determination circuit.
pub trait KeyscanDriver {
    /// Scans a key and returns the delta from the previous key scan
    async fn scan(&mut self, callback: impl FnMut(KeyChangeEvent));
}
