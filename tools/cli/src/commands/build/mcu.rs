pub struct McuConfig {
    pub target: &'static str,
    pub uf2_family_id: u32,
    pub uf2_start_addr: u32,
}

pub const MCU_CONFIG_RP2040: McuConfig = McuConfig {
    target: "thumbv6m-none-eabi",
    uf2_family_id: 0xe48bff56,
    uf2_start_addr: 0x10000000,
};

pub const MCU_CONFIG_NRF52840: McuConfig = McuConfig {
    target: "thumbv7em-none-eabihf",
    uf2_family_id: 0xADA52840,
    uf2_start_addr: 0x26000,
};
