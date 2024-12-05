use usbd_hid::descriptor::MediaKeyboardReport;

use crate::keycode::KeyCode;

mod reporter;

pub struct MediaKeyboardState {
    reporter: reporter::MediaKeyboardReportGenerator,
}

impl MediaKeyboardState {
    pub fn new() -> Self {
        Self {
            reporter: reporter::MediaKeyboardReportGenerator::new(),
        }
    }
}

pub struct MediaKeyboardLocalState {
    pub media_key: Option<u16>,
}

impl MediaKeyboardLocalState {
    pub fn new() -> Self {
        Self { media_key: None }
    }

    pub fn process_event(&mut self, kc: &KeyCode) {
        match kc {
            KeyCode::Media(key) => {
                self.media_key = Some(*key as u16);
            }
            _ => {}
        };
    }

    pub fn report(self, global_state: &mut MediaKeyboardState) -> Option<MediaKeyboardReport> {
        global_state.reporter.generate(self.media_key)
    }
}
