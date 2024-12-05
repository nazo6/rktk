<h1 align="center">rktk</h1>
<p align="center">Rust Keyboard Toolkit</p>

rktk is a keyboard firmware framework written in Rust.

Currently, rktk supports nRF52840 and RP2040, and although there are bugs and
performance issues (especially related to split keyboards), it can be used as a
decent keyboard.

This firmware consists of two parts: the core functionality implemented in the
`rktk` crate and the driver that actually interacts with the hardware. This
makes it easy to extend.

## Crates

| Crate                  | crates.io                                                                                                                       | docs.rs                                                                                                    |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| rktk                   | [![Crates.io Version](https://img.shields.io/crates/v/rktk)](https://crates.io/crates/rktk)                                     | [![docs.rs](https://img.shields.io/docsrs/rktk)](https://docs.rs/rktk)                                     |
| rktk-drivers-common    | [![Crates.io Version](https://img.shields.io/crates/v/rktk-drivers-common)](https://crates.io/crates/rktk-drivers-common)       | [![docs.rs](https://img.shields.io/docsrs/rktk-drivers-common)](https://docs.rs/rktk-drivers-common)       |
| rktk-drivers-nrf       | [![Crates.io Version](https://img.shields.io/crates/v/rktk-drivers-nrf)](https://crates.io/crates/rktk-drivers-nrf)             | [![docs.rs](https://img.shields.io/docsrs/rktk-drivers-nrf)](https://docs.rs/rktk-drivers-nrf)             |
| rktk-drivers-rp        | [![Crates.io Version](https://img.shields.io/crates/v/rktk-drivers-rp)](https://crates.io/crates/rktk-drivers-rp)               | [![docs.rs](https://img.shields.io/docsrs/rktk-drivers-rp)](https://docs.rs/rktk-drivers-rp)               |
| rktk-keymanager        | [![Crates.io Version](https://img.shields.io/crates/v/rktk-keymanager)](https://crates.io/crates/rktk-keymanager)               | [![docs.rs](https://img.shields.io/docsrs/rktk-keymanager)](https://docs.rs/rktk-keymanager)               |
| rktk-cli               | [![Crates.io Version](https://img.shields.io/crates/v/rktk-cli)](https://crates.io/crates/rktk-cli)                             |                                                                                                            |
| rktk-rrp               | [![Crates.io Version](https://img.shields.io/crates/v/rktk-rrp)](https://crates.io/crates/rktk-rrp)                             | [![docs.rs](https://img.shields.io/docsrs/rktk-rrp)](https://docs.rs/rktk-rrp)                             |
| rktk-rrp-client-webhid | [![Crates.io Version](https://img.shields.io/crates/v/rktk-rrp-client-webhid)](https://crates.io/crates/rktk-rrp-client-webhid) | [![docs.rs](https://img.shields.io/docsrs/rktk-rrp-client-webhid)](https://docs.rs/rktk-rrp-client-webhid) |

## Features

- ✅ : Working
- 🔵 : Almost working
- 🟡 : Partially working, missing some features
- 🔴 : Planned.
- ❌ : Not planned.
- (blank): Not needed.

### Core features

| Feature            | Status                          |
| ------------------ | ------------------------------- |
| Keyscan            | ✅                              |
| Mouse              | ✅                              |
| Encoder            | 🔵                              |
| Key mapping        | 🟡 (See below table for detail) |
| Hook system        | 🟡                              |
| Split keyboard     | ✅                              |
| Non-Split keyboard | 🔵                              |
| Display            | 🟡                              |
| RGB led            | 🟡                              |
| USB                | ✅                              |
| Bluetooth          | 🟡                              |
| Remapper support   | 🟡                              |
| Double-tap reset   | ✅                              |

#### Key mapping features

Key mapping features is implemented in `rktk-keymanager` and this crate does not
depend on rktk or embassy. Keymap is defined as normal two-dimensional array.
See [keyball61's keymap](./keyboards/keyball-common/src/keymap.rs) for example.

| Feature name           | Status | Note                                                  |
| ---------------------- | ------ | ----------------------------------------------------- |
| **Key action**         |        |                                                       |
| Mod-Tap                | ✅     | Unlike QMK, any keycode can be specified as modifier. |
| Tap-Hold               | ✅     |                                                       |
| Tap Dance              | ✅     |                                                       |
| Oneshot key            | ✅     |                                                       |
| Combo key              | 🟡     |                                                       |
| &nbsp;                 |        |                                                       |
| **Key code**           |        |                                                       |
| Normal key             | ✅     |                                                       |
| Modifier key           | ✅     |                                                       |
| Media key              | ✅     |                                                       |
| Mouse key              | ✅     |                                                       |
| Mouse scroll momentary | ✅     |                                                       |
| Layer momentary (MO)   | ✅     |                                                       |
| Layer toggle (TG)      | ✅     |                                                       |

### Drivers

- "Common" means that the driver is implemented in `rktk-drivers-common`.
  Drivers implemented in `rktk-drivers-common` use embassy traits, so they can
  be easily ported to various platforms.

| Driver                     | Common | RP2040    | NRF52840        |
| -------------------------- | ------ | --------- | --------------- |
| **Key scanner**            |        |           |                 |
| Matrix                     | 🔴     | 🔴        | 🔴              |
| Matrix with shift register | 🔵     | 🔴        | 🔵              |
| (Japanese) Duplex-Matrix   | ✅     | ✅        | ✅              |
| &nbsp;                     |        |           |                 |
| **Mouse**                  |        |           |                 |
| PMW3360                    | ✅     | ✅        | ✅              |
| PAW3395                    | ✅     | ✅        | ✅              |
| &nbsp;                     |        |           |                 |
| **Host communication**     |        |           |                 |
| USB                        | ✅     | ✅        | ✅              |
| Bluetooth                  | ❌     | ❌        | ✅ (SoftDevice) |
| &nbsp;                     |        |           |                 |
| **Display**                |        |           |                 |
| SSD1306                    | ✅     | ✅        | ✅              |
| &nbsp;                     |        |           |                 |
| **Split**                  |        |           |                 |
| Half-duplex single wire    |        | ✅ (PIO)  | ✅ (UART)       |
| Full-duplex dual wire      |        | 🔴 (UART) | 🔴 (UART)       |
| Bluetooth                  |        | ❌        | 🔴              |
| &nbsp;                     |        |           |                 |
| **RGB led**                |        |           |                 |
| WS2812                     |        | ✅ (PIO)  | ✅ (PWM)        |
| &nbsp;                     |        |           |                 |
| **Double-tap reset**       |        | ✅        |                 |

## Examples

You can find examples in the `examples` directory.

## Development

See `cargo rktk --help` and `.vscode/tasks.json` to build firmware and start
development.

### Dependencies

You need to install some tools to generate firmware.

- `arm-none-eabi-objcopy`: Required to generate uf2 file.
- `wasm-pack`: Required to build rktk-client

### MSRV

rktk actually doesn't depends on nightly feature of _rustc_, but uses nightly
cargo features like `per-package-target`. So, it requires nightly toolchain.

### Creating new keyboard

Currently, there is no guide for building a new keyboard, but you can refer to
the examples in this repo or the following repository:

- https://github.com/nazo6/rktk-neg
- https://github.com/nazo6/rktk-keyball-rs

## Credits & Acknowledgements

- [rumcake](https://github.com/Univa/rumcake): RP2040 double-tap-reset driver
- [uf2](https://github.com/microsoft/uf2): uf2conv.py, uf2families.json
- [rust-dilemma](https://github.com/simmsb/rusty-dilemma): RP2040 Half-duplex
  communication
- [qmk](https://github.com/qmk/qmk_firmware): RP2040 Half-duplex communication
- [rmk](https://github.com/HaoboGu/rmk): bluetooth implemention
