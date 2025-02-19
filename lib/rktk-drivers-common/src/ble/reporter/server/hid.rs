use trouble_host::{
    prelude::{FixedGattValue, GattValue},
    types::gatt_traits::FromGattError,
};
use usbd_hid::descriptor::generator_prelude::*;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (report_id = 0x01,) = {
            (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
                #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
            };
            (usage_min = 0x00, usage_max = 0xFF) = {
                #[item_settings constant,variable,absolute] reserved=input;
            };
            (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
                #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
            };
            (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
                #[item_settings data,array,absolute] keycodes=input;
            };
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (report_id = 0x02,) = {
                (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                    #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
                };
                (usage_page = GENERIC_DESKTOP,) = {
                    (usage = X,) = {
                        #[item_settings data,variable,relative] x=input;
                    };
                    (usage = Y,) = {
                        #[item_settings data,variable,relative] y=input;
                    };
                    (usage = WHEEL,) = {
                        #[item_settings data,variable,relative] wheel=input;
                    };
                };
                (usage_page = CONSUMER,) = {
                    (usage = AC_PAN,) = {
                        #[item_settings data,variable,relative] pan=input;
                    };
                };
            };
        };
    },
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        (report_id = 0x03,) = {
            (usage_page = CONSUMER, usage_min = 0x00, usage_max = 0x514) = {
            #[item_settings data,array,absolute,not_null] media_usage_id=input;
            }
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = SYSTEM_CONTROL) = {
        (report_id = 0x04,) = {
            (usage_min = 0x81, usage_max = 0xB7, logical_min = 1) = {
                #[item_settings data,array,absolute,not_null] system_usage_id=input;
            };
        };
    },
)]
#[allow(dead_code)]
pub struct BleKeyboardReport {
    pub modifier: u8,
    pub reserved: u8,
    pub leds: u8,
    pub keycodes: [u8; 6],
    pub buttons: u8,
    pub x: i8,
    pub y: i8,
    pub wheel: i8,
    pub pan: i8,
    pub media_usage_id: u16,
    pub system_usage_id: u8,
    pub vial_input_data: [u8; 32],
    pub vial_output_data: [u8; 32],
}

pub struct Desc([u8; 180]);

pub const fn const_desc() -> Desc {
    Desc([
        5u8, 1u8, 9u8, 6u8, 161u8, 1u8, 133u8, 1u8, 5u8, 7u8, 25u8, 224u8, 41u8, 231u8, 21u8, 0u8,
        37u8, 1u8, 117u8, 1u8, 149u8, 8u8, 129u8, 2u8, 25u8, 0u8, 41u8, 255u8, 38u8, 255u8, 0u8,
        117u8, 8u8, 149u8, 1u8, 129u8, 3u8, 5u8, 8u8, 25u8, 1u8, 41u8, 5u8, 37u8, 1u8, 117u8, 1u8,
        149u8, 5u8, 145u8, 2u8, 149u8, 3u8, 145u8, 3u8, 5u8, 7u8, 25u8, 0u8, 41u8, 221u8, 38u8,
        255u8, 0u8, 117u8, 8u8, 149u8, 6u8, 129u8, 0u8, 192u8, 5u8, 1u8, 9u8, 2u8, 161u8, 1u8, 9u8,
        1u8, 161u8, 0u8, 133u8, 2u8, 5u8, 9u8, 25u8, 1u8, 41u8, 8u8, 37u8, 1u8, 117u8, 1u8, 149u8,
        8u8, 129u8, 2u8, 5u8, 1u8, 9u8, 48u8, 23u8, 129u8, 255u8, 255u8, 255u8, 37u8, 127u8, 117u8,
        8u8, 149u8, 1u8, 129u8, 6u8, 9u8, 49u8, 129u8, 6u8, 9u8, 56u8, 129u8, 6u8, 5u8, 12u8, 10u8,
        56u8, 2u8, 129u8, 6u8, 192u8, 192u8, 5u8, 12u8, 9u8, 1u8, 161u8, 1u8, 133u8, 3u8, 5u8,
        12u8, 25u8, 0u8, 42u8, 20u8, 5u8, 21u8, 0u8, 39u8, 255u8, 255u8, 0u8, 0u8, 117u8, 16u8,
        129u8, 0u8, 192u8, 5u8, 1u8, 9u8, 128u8, 161u8, 1u8, 133u8, 4u8, 25u8, 129u8, 41u8, 183u8,
        21u8, 1u8, 38u8, 255u8, 0u8, 117u8, 8u8, 129u8, 0u8, 192u8,
    ])
}

impl FixedGattValue for Desc {
    const SIZE: usize = 180;

    fn from_gatt(data: &[u8]) -> Result<Self, FromGattError> {
        <[u8; 180] as GattValue>::from_gatt(data).map(Desc)
    }

    fn to_gatt(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Default for Desc {
    fn default() -> Self {
        const_desc()
    }
}

#[allow(unused)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BleCompositeReportType {
    Keyboard = 0x01,
    Mouse = 0x02,
    Media = 0x03,
    // System = 0x04,
}
