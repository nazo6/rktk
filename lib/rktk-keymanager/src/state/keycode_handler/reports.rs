#[derive(Debug, PartialEq)]
pub struct Reports {
    pub keyboard_report: Option<KeyboardReport>,
    pub media_keyboard_report: Option<MediaKeyboardReport>,
    pub mouse_report: Option<MouseReport>,
    pub transparent_report: TransparentReport,
}

impl Default for Reports {
    fn default() -> Self {
        Self {
            keyboard_report: None,
            media_keyboard_report: None,
            mouse_report: None,
            transparent_report: TransparentReport {
                flash_clear: false,
                ble_bond_clear: false,
                output: Output::Usb,
                bootloader: false,
            },
        }
    }
}
