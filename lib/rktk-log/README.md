# rktk-log

logger for rktk

## Credits

- Most code are from [defmt-or-log](https://github.com/t-moe/defmt-or-log).
- defmtusb code are from [defmtusb](https://github.com/micro-rust/defmtusb).

## About

This crate unifies rktk logs and allows flexible switching between log and defmt
using feature flag.

Also, this crate provides `defmtusb` feature, which provides
`defmt::global_logger` to send defmt log via usb. This feature does not work
without usb driver implementations. To use it, enable `defmtusb` feature of
`rktk-drivers-common` crate.
