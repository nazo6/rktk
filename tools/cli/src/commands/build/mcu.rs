pub struct McuConfig {
    pub target: &'static str,
    pub uf2_family_id: u32,
    pub uf2_start_addr: u32,
}

macro_rules! gen_mcu {
    ($($name:ident: $val:tt,)*) => {
        paste::paste! {
            #[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq, serde::Deserialize)]
            pub enum BuildMcuList {
                $([<$name>],)*
            }

            impl BuildMcuList {
                pub fn get_mcu_config(&self) -> &'static McuConfig {
                    match self {
                        $(BuildMcuList::$name => &[<MCU_ $name:snake:upper>],)*
                    }
                }
            }
        }

        $(
            gen_mcu!(@mcu, $name: $val,);
        )*
    };
    (@mcu, $name:ident: $val:tt,) => {
        paste::paste! {
            pub static [<MCU_ $name:snake:upper>]: McuConfig = McuConfig $val;
        }
    }
}

gen_mcu! {
    Rp2040: {
        target: "thumbv6m-none-eabi",
        uf2_family_id: 0xe48bff56,
        uf2_start_addr: 0x10000000,
    },
    Nrf52840: {
        target: "thumbv7em-none-eabihf",
        uf2_family_id: 0xADA52840,
        uf2_start_addr: 0x26000,
    },
    Nrf52840SDV7: {
        target: "thumbv7em-none-eabihf",
        uf2_family_id: 0xADA52840,
        uf2_start_addr: 0x27000,
    },
}
