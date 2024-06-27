use usbd_hid::descriptor::MediaKeyboardReport;

pub struct MediaKeyboardReportGenerator {
    empty_mkb_sent: bool,
    previous_mkb: Option<MediaKeyboardReport>,
}

impl MediaKeyboardReportGenerator {
    pub fn new() -> Self {
        Self {
            empty_mkb_sent: false,
            previous_mkb: None,
        }
    }

    pub fn gen(&mut self, media_usage_id: Option<u16>) -> Option<MediaKeyboardReport> {
        match media_usage_id {
            Some(key) => {
                self.empty_mkb_sent = false;
                let mut same = false;
                if let Some(prev) = &self.previous_mkb {
                    if prev.usage_id == key {
                        same = true;
                    }
                }
                if same {
                    None
                } else {
                    self.previous_mkb = Some(MediaKeyboardReport { usage_id: key });
                    Some(MediaKeyboardReport { usage_id: key })
                }
            }
            None => {
                self.previous_mkb = None;
                if !self.empty_mkb_sent {
                    self.empty_mkb_sent = true;
                    Some(MediaKeyboardReport { usage_id: 0 })
                } else {
                    None
                }
            }
        }
    }
}
