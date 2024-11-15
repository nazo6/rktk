<h1 align="center">rktk</h1>
<p align="center">Rust Keyboard Toolkit</p>

rktk is a keyboard firmware framework written in Rust.

Initially, it was a firmware for the
[keyball](https://github.com/Yowkees/keyball), but a driver system was
introduced to make it easy to extend to various keyboards.

Currently, it supports nRF52840 and RP2040, and although there are bugs and
performance issues (especially related to split keyboards), it can be used as a
decent keyboard.

This firmware consists of two parts: the core functionality implemented in the
`rktk` crate and the driver that actually interacts with the hardware. This
makes it easy to extend.

## Features

- ✅ : Working
- 🟡 : WIP, partly implemented.
- 🔴 : Planned.
- ❌ : Not planned.
- (blank): Not needed.

### Core features

| Feature                                  | Status |
| ---------------------------------------- | ------ |
| Keyscan                                  | ✅     |
| Mouse                                    | ✅     |
| Advanced key mapping system (layer etc.) | 🟡     |
| Split keyboard                           | ✅     |
| Non-Split keyboard                       | 🟡     |
| Display                                  | 🟡     |
| Backlight                                | 🟡     |
| USB                                      | ✅     |
| Bluetooth                                | 🟡     |
| Remapper support                         | 🟡     |
| Double-tap reset                         | ✅     |

#### Key mapping features

Key mapping features is implemented in `rktk-keymanager` and this crate does not
depend on rktk or embassy. Keymap is defined as normal two-dimensional array.
See [keyball61's keymap](./keyboards/keyball-common/src/keymap.rs) for example.

| Feature name           | Status |                                                         |
| ---------------------- | ------ | ------------------------------------------------------- |
| &nbsp;                 |        |                                                         |
| **Key action**         |        |                                                         |
| Mod-Tap                | ✅     | Unlike QMK, any keycode can be specified as modifier.   |
| Tap-Hold               | 🟡     | Currently, this behaves like `HOLD_ON_OTHER_KEY_PRESS`. |
| Tap Dance              | ✅     |                                                         |
| Oneshot key            | ✅     |                                                         |
| &nbsp;                 |        |                                                         |
| **KeyCode**            |        |                                                         |
| Normal key             | ✅     |                                                         |
| Modifier key           | ✅     |                                                         |
| Media key              | ✅     |                                                         |
| Mouse key              | ✅     |                                                         |
| Mouse scroll momentary | ✅     |                                                         |
| Layer momentary (MO)   | ✅     |                                                         |
| Layer toggle (TG)      | ✅     |                                                         |

### Drivers

- "Common" means that the driver is implemented in `rktk-drivers-common`.
  Drivers implemented in `rktk-drivers-common` use embassy traits, so they can
  be easily ported to various platforms.

| Driver                     | Common | RP2040    | NRF52840  |
| -------------------------- | ------ | --------- | --------- |
| **Key scanner**            |        |           |           |
| Matrix                     | 🔴     | 🔴        | 🔴        |
| Matrix with shift register | 🟡     | 🔴        | 🟡        |
| (Japanese) Duplex-Matrix   | ✅     | ✅        | ✅        |
| &nbsp;                     |        |           |           |
| **Mouse**                  |        |           |           |
| PMW3360                    | ✅     | ✅        | ✅        |
| PAW3395                    | ✅     | ✅        | ✅        |
| &nbsp;                     |        |           |           |
| **Host communication**     |        |           |           |
| USB                        | ✅     | ✅        | ✅        |
| Bluetooth                  | ❌     | ❌        | ✅        |
| &nbsp;                     |        |           |           |
| **Display**                |        |           |           |
| SSD1306                    | ✅     | ✅        | ✅        |
| &nbsp;                     |        |           |           |
| **Split**                  |        |           |           |
| Half-duplex single wire    | ❌     | ✅ (PIO)  | ✅ (UART) |
| Full-duplex dual wire      | ❌     | 🔴 (UART) | 🔴 (UART) |
| Bluetooth                  | ❌     | ❌        | 🔴        |
| &nbsp;                     |        |           |           |
| **Backlight**              |        |           |           |
| WS2812                     |        | ✅ (PIO)  | ✅ (PWM)  |
| &nbsp;                     |        |           |           |
| **Double-tap reset**       |        | ✅        |           |

## Development

See `cargo x --help` and `.vscode/tasks.json` to build firmware and start
development.

### Dependencies

You need to install some tools to generate firmware.

- [elf2uf2-rs](https://github.com/JoNil/elf2uf2-rs): Required to generate uf2
  firmware for RP2040
- `arm-none-eabi-objcopy` and `python3`: Required to generate uf2 firmware for
  nRF52840
- `wasm-pack`: Required to build rktk-client

### MSRV

rktk actually doesn't depends on nightly feature of _rustc_, but uses nightly
cargo features like `per-package-target`. So, it requires nightly toolchain.

### Creating new keyboard

See [keyball61-rp2040](./keyboards/keyball61-rp2040) and
[keyball61-nrf52840](./keyboards/keyball61-nrf52840) for example.

As normal matrix keyscan driver is not implemented yet, it is not possible to
create a new keyboard without implementing a new driver.

## Credits & Acknowledgements

- [rumcake](https://github.com/Univa/rumcake): RP2040 double-tap-reset driver
- [uf2](https://github.com/microsoft/uf2): uf2conv.py, uf2families.json
- [rust-dilemma](https://github.com/simmsb/rusty-dilemma): RP2040 Half-duplex
  communication
- [qmk](https://github.com/qmk/qmk_firmware): RP2040 Half-duplex communication
- [rmk](https://github.com/HaoboGu/rmk): bluetooth implemention
