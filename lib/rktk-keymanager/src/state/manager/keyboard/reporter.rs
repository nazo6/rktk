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

    pub fn gen(
        &mut self,
        keycodes: &heapless::FnvIndexSet<u8, 8>,
        modifier: u8,
    ) -> Option<KeyboardReport> {
        if modifier == 0 && keycodes.is_empty() {
            if !self.empty_kb_sent {
                self.empty_kb_sent = true;
                Some(KeyboardReport::default())
            } else {
                None
            }
        } else {
            let mut keycodes_array = [0; 6];
            for (i, kc) in keycodes.iter().take(6).enumerate() {
                keycodes_array[i] = *kc;
            }

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
