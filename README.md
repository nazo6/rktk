# rktk

Rust keybord toolkit.

Currently, the firmware is for the Keyball61, but by separating the core
functionality from the driver, the core functionality is architecture- and
hardware-independent and can be easily extended.

Our current goal is to port it to the NRF52840 chip and connect it via
bluetooth.

## Features

- ✅ : Working
- 🟡 : WIP, partly implemented.
- 🔴 : WIP, not working yet.
- ❌ : Only planned. Not implemented yet.
- Empty: Not planned/Not needed.

### Core features

| Feature            | Status |
| ------------------ | ------ |
| Keyscan            | ✅     |
| Media key support  | ✅     |
| Mouse              | ✅     |
| Layer system       | 🟡     |
| Split keyboard     | ✅     |
| Non-Split keyboard | 🟡     |
| Display            | 🟡     |
| Backlight LED      | 🟡     |
| USB                | ✅     |
| Bluetooth          | 🟡     |
| Remapper support   | ❌     |
| Double-tap reset   | ✅     |

### Drivers

#### Key scanner

| Driver                | RP2040 | NRF52840 |
| --------------------- | ------ | -------- |
| Duplex-Matrix Scanner | ✅     | ✅       |

#### Mouse

| Driver  | RP2040 | NRF52840 |
| ------- | ------ | -------- |
| PMW3360 | ✅     | ✅       |

#### Display

| Driver  | RP2040 | NRF52840 |
| ------- | ------ | -------- |
| SSD1306 | ✅     | ✅       |

#### Split

| Driver                         | RP2040 | NRF52840 |
| ------------------------------ | ------ | -------- |
| Single wire Half-duplex (PIO)  | ✅     |          |
| Single wire Half-duplex (UART) |        | 🟡       |

#### Backlight

| Driver       | RP2040 | NRF52840 |
| ------------ | ------ | -------- |
| WS2812 (PIO) | ✅     |          |
| WS2812 (PWM) |        | 🟡       |

#### USB

| RP2040 | NRF52840 |
| ------ | -------- |
| ✅     | ✅       |

#### Bluetooth

| Driver         | RP2040 | NRF52840 |
| -------------- | ------ | -------- |
| NRF Softdevice |        | 🟡       |

#### Double tap reset

| RP2040 | NRF52840 |
| ------ | -------- |
| ✅     |          |

## Development

See `cargo x --help` and `.vscode/tasks.json` to build firmware and start
development.

### Dependencies

You need to install some tools to generate firmware.

- [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs): Required to generate uf2
  firmware for RP2040
- `arm-none-eabi-objcopy` and `python3`: Required to generate uf2 firmware for
  nRF52840
- `wasm-pack`: Required to build rrp-web

## MSRV

rktk actually doesn't depends on nightly feature of _rustc_, but uses nightly
cargo features like `per-package-target`. So, it requires nightly toolchain.

## Architecture

There are `rktk`, `rktk-drivers-common`, `rktk-drivers-{rp2040,nrf52}` and a
crate for each keyboard.

The `rktk` crate is completely hardware independent and provides the core
functionality of the keyboard.

The `rktk-drivers-common` uses the abstraction of embedded-hal and
embedded-hal-async to provide the basis for drivers that can be used universally
on a variety of chips. This makes porting drivers to various chips very easy.

The `rktk-drivers-*` crate provides drivers for each chip. Most drivers are
wrappings of `rktk-drivers-common`, but some are proprietary implementations,
such as `ws2812-pio`.

Each keyboard crate can then create a driver for the appropriate chip and pass
it to `rktk::task::start` to configure the actual working keyboard firmware. The
only keyboard that currently works is `keyball61-rp2040`, but it is not too
difficult to create your own keyboard by referring to the
`keyboards/keyball61-rp2040` directory.

## Credits

- [rumcake](https://github.com/Univa/rumcake) (rp2040 double-tap-to-bootloader
  driver)
- [uf2](https://github.com/microsoft/uf2) (uf2conv.py, uf2families.json)
- ARM GNU Toolchain (arm-none-eabi-objcopy)
- [rust-dilemma](https://github.com/simmsb/rusty-dilemma/blob/5ffe8f5d2b6b0d534a4309edc737364cd96f44f1/firmware/src/interboard/onewire.rs)
  and
  [qmk](https://github.com/qmk/qmk_firmware/blob/master/platforms/chibios/drivers/vendor/RP/RP2040/serial_vendor.c)
  for pio half-duplex
- [rmk](https://github.com/HaoboGu/rmk) for bluetooth implemention
