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

    pub fn gen(&mut self, keycodes: &[u8], modifier: u8) -> Option<KeyboardReport> {
        if modifier == 0 && keycodes.is_empty() {
            if !self.empty_kb_sent {
                self.empty_kb_sent = true;
                Some(KeyboardReport::default())
            } else {
                None
            }
        } else {
            let keycode_len = keycodes.len().min(6);
            let mut keycodes_array = [0; 6];
            keycodes_array[..keycode_len].copy_from_slice(&keycodes[..keycode_len]);

            self.empty_kb_sent = false;
            Some(KeyboardReport {
                keycodes: keycodes_array,
                modifier,
                reserved: 0,
                leds: 0,
            })
        }
    }
}
