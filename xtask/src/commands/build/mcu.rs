pub struct McuConfig {
    pub target: &'static str,
}

pub const MCU_CONFIG_RP2040: McuConfig = McuConfig {
    target: "thumbv6m-none-eabi",
};

pub const MCU_CONFIG_NRF52840: McuConfig = McuConfig {
    target: "thumbv7em-none-eabihf",
};
