use usbd_hid::descriptor::MouseReport;

use crate::config::{SCROLL_DIVIDER_X, SCROLL_DIVIDER_Y};

pub struct MouseReportGenerator {
    empty_mouse_sent: bool,
    // scoll_modeがonのときにSomeとなり、中身には「残っているスクロール」の値が入る。
    // スクロールは値が小さい関係上、1より小さい値になることが多々ある。これを0とみなすと、小さいスクロールができなくなってしまう。
    scroll_remained: Option<(i8, i8)>,
}

impl MouseReportGenerator {
    pub fn new() -> Self {
        Self {
            empty_mouse_sent: false,
            scroll_remained: None,
        }
    }

    pub fn gen(
        &mut self,
        mouse_event: (i8, i8),
        mouse_buttons: u8,
        scroll_mode: bool,
    ) -> Option<MouseReport> {
        if scroll_mode {
            if self.scroll_remained.is_none() {
                self.scroll_remained = Some((0, 0));
            }
        } else {
            self.scroll_remained = None;
        }

        if mouse_event == (0, 0) && mouse_buttons == 0 {
            if !self.empty_mouse_sent {
                self.empty_mouse_sent = true;
                Some(MouseReport {
                    x: 0,
                    y: 0,
                    buttons: 0,
                    wheel: 0,
                    pan: 0,
                })
            } else {
                None
            }
        } else {
            self.empty_mouse_sent = false;
            if let Some((remained_wheel, remained_pan)) = &mut self.scroll_remained {
                let wheel_raw = mouse_event.0 + *remained_wheel;
                let pan_raw = mouse_event.1 + *remained_pan;
                let wheel = wheel_raw / SCROLL_DIVIDER_Y;
                let pan = pan_raw / SCROLL_DIVIDER_X;
                *remained_wheel = wheel_raw % SCROLL_DIVIDER_Y;
                *remained_pan = pan_raw % SCROLL_DIVIDER_X;
                Some(MouseReport {
                    x: 0,
                    y: 0,
                    buttons: mouse_buttons,
                    wheel,
                    pan,
                })
            } else {
                Some(MouseReport {
                    x: mouse_event.1,
                    y: mouse_event.0,
                    buttons: mouse_buttons,
                    wheel: 0,
                    pan: 0,
                })
            }
        }
    }
}
