use usbd_hid::descriptor::KeyboardReport;

pub struct KeyboardReportGenerator {
    empty_kb_sent: bool,
}

impl KeyboardReportGenerator {
    pub fn new() -> Self {
        Self {
            empty_kb_sent: false,
        }
    }

    #[allow(clippy::get_first)]
    pub fn gen(&mut self, keycodes: &heapless::Vec<u8, 6>, modifier: u8) -> Option<KeyboardReport> {
        if modifier == 0 && keycodes.is_empty() {
            if !self.empty_kb_sent {
                self.empty_kb_sent = true;
                Some(KeyboardReport::default())
            } else {
                None
            }
        } else {
            let keycodes: [u8; 6] = [
                keycodes.get(0).copied().unwrap_or(0),
                keycodes.get(1).copied().unwrap_or(0),
                keycodes.get(2).copied().unwrap_or(0),
                keycodes.get(3).copied().unwrap_or(0),
                keycodes.get(4).copied().unwrap_or(0),
                keycodes.get(5).copied().unwrap_or(0),
            ];
            self.empty_kb_sent = false;
            Some(KeyboardReport {
                keycodes,
                modifier,
                reserved: 0,
                leds: 0,
            })
        }
    }
}
