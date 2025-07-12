<h1 align="center">rktk</h1>
<p align="center">Rust Keyboard Toolkit</p>

[![Crates.io Version](https://img.shields.io/crates/v/rktk)](https://crates.io/crates/rktk)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/nazo6/rktk/lib.yml)
[![Doc build status](https://img.shields.io/github/actions/workflow/status/nazo6/rktk/doc.yml?label=doc)](https://rktk-docs.nazo6.dev)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/nazo6/rktk)

rktk is a keyboard firmware framework written in Rust.

Currently, rktk supports nRF52840 and RP2040, and although there are bugs and
performance issues (especially related to split keyboards), it can be used as a
decent keyboard.

This firmware consists of two parts: the core functionality implemented in the
`rktk` crate and the driver that actually interacts with the hardware. This
makes it easy to extend.

## Docs

- [Main site](https://rktk.nazo6.dev/): Very incomplete documentation site
- [API Docs](https://rktk-docs.nazo6.dev/): RKTK Rust API docs
- [Deepwiki](https://deepwiki.com/nazo6/rktk): It contains some incorrect
  content, but considering it is AI-generated, it is excellent.

### Examples

You can find examples in the `examples` directory.

Also, you can find some advanced examples in the following repositories:

- https://github.com/nazo6/rktk-neg
- https://github.com/nazo6/rktk-keyball-rs

## Features

- ✅ : Working
- 🟡 : Basic implementation only or known bugs.
- 🔴 : Planned.
- (blank): N/A.

### Core features

| Feature                | Status | Note                       |
| ---------------------- | ------ | -------------------------- |
| Keyscan                | ✅     |                            |
| Key mapping            | 🟡     | See below table for detail |
| Mouse                  | ✅     |                            |
| Encoder                | 🟡     |                            |
| Hook system            | 🟡     |                            |
| USB                    | ✅     |                            |
| Bluetooth              | 🟡     |                            |
| Split keyboard         | ✅     |                            |
| Display                | 🟡     |                            |
| Storage                | 🟡     |                            |
| RGB led                | 🟡     |                            |
| Remapper (rktk-client) | 🟡     |                            |

#### Key mapping features

Key mapping features is implemented in `rktk-keymanager` and this crate does not
depend on rktk or embassy.

| Feature name           | Status | Note                                      |
| ---------------------- | ------ | ----------------------------------------- |
| **Key action**         |        |                                           |
| Tap-Hold               | ✅     | Called as `Mod-Tap` or `Layer-Tap` in QMK |
| Tap Dance              | ✅     |                                           |
| Oneshot key            | ✅     |                                           |
| Combo key              | 🟡     |                                           |
| **Key code**           |        |                                           |
| Normal key             | ✅     |                                           |
| Modifier key           | ✅     |                                           |
| Media key              | ✅     |                                           |
| Mouse key              | ✅     |                                           |
| Mouse scroll momentary | ✅     |                                           |
| Layer momentary (MO)   | ✅     |                                           |
| Layer toggle (TG)      | ✅     |                                           |

### Drivers

Driver that is available in the `rktk-drivers-common` crate is available for all
platforms which have embassy compatible HAL.

| Driver                         | Common       | RP2040   | NRF52840        |
| ------------------------------ | ------------ | -------- | --------------- |
| **Key scanner**                |              |          |                 |
| Matrix                         | 🟡           | -        | -               |
| Matrix with shift register     | ✅           | -        | -               |
| (Japanese) Duplex-Matrix       | 🟡           | -        | -               |
| &nbsp;                         |              |          |                 |
| **Mouse**                      |              |          |                 |
| PMW3360                        | ✅           | -        | -               |
| PAW3395                        | ✅           | -        | -               |
| &nbsp;                         |              |          |                 |
| **Encoder**                    | 🟡           | -        | -               |
| &nbsp;                         |              |          |                 |
| **Debouncer**                  |              |          |                 |
| Eager debouncer                | 🟡           | -        | -               |
| &nbsp;                         |              |          |                 |
| **Host communication**         |              |          |                 |
| USB                            | ✅           | -        | -               |
| Bluetooth                      | 🟡 (Trouble) |          | 🟡 (SoftDevice) |
| &nbsp;                         |              |          |                 |
| **Split communication**        |              |          |                 |
| Half-duplex (single wire, TRS) |              | 🟡 (PIO) | 🟡 (UART)       |
| Full-duplex (dual wire, TRRS)  |              |          | ✅ (UART)       |
| Bluetooth                      |              |          | 🔴              |
| &nbsp;                         |              |          |                 |
| **Display**                    |              |          |                 |
| SSD1306                        | ✅           | -        | -               |
| &nbsp;                         |              |          |                 |
| **Storage**                    |              |          |                 |
| sequential-storage (NorFlash)  | 🟡           | -        | -               |
| &nbsp;                         |              |          |                 |
| **RGB led**                    |              |          |                 |
| WS2812                         |              | ✅ (PIO) | ✅ (PWM)        |

## Development

### Dependencies

In addition to the dependencies specified [here](https://rktk.nazo6.dev/docs),
to develop rktk, you need to install the following dependencies:

- [cargo-hack](https://github.com/taiki-e/cargo-hack): To run matrix check
- [dioxus cli](https://crates.io/crates/dioxus-cli): To run rktk-client
- [cargo-check-delta](https://github.com/nazo6/cargo-check-delta): Used by
  rust-analyzer

### MSRV

#### As library

Unless you enable a specific feature, rktk's MSRV is the latest stable version
of Rust.

In addition, a nightly compiler is required to minimize the binary size.

#### For development

As a library, rktk does not depend on unstable feature, but the rktk repository
workspace depends on cargo's unstable feature. Therefore, nightly is required to
develop rktk.

## Credits & Acknowledgements

- [rumcake](https://github.com/Univa/rumcake): RP2040 double-tap-reset driver
- [rust-dilemma](https://github.com/simmsb/rusty-dilemma): RP2040 Half-duplex
  communication
- [qmk](https://github.com/qmk/qmk_firmware): RP2040 Half-duplex communication
- [rmk](https://github.com/HaoboGu/rmk): BLE implemention
