pub const RRP_HID_BUFFER_SIZE: usize = 32;

use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = 0xFF70, usage = 0x71) = {
        (usage = 0x72, logical_min = 0x0) = {
            #[item_settings data,variable,absolute] input_data=input;
        };
        (usage = 0x73, logical_min = 0x0) = {
            #[item_settings data,variable,absolute] output_data=output;
        };
    }
)]
pub struct RrpReport {
    pub input_data: [u8; 32],
    pub output_data: [u8; 32],
}
