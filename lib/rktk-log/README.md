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

## Viewing defmtusb log

By using
[my defmt-print fork](https://github.com/nazo6/defmt/tree/defmt-print-serial),
you can print log from usb (serialport).

### Usage

```sh
# Install
cargo install --git https://github.com/nazo6/defmt --branch defmt-print-serial defmt-print

# Connect
defmt-print <elf binary path> serial COM1 # Change COM1 to your serial port
```

To find com port, you can use software such as
[USB Device Tree Viewer](https://www.uwe-sieber.de/usbtreeview_e.html) on
windows.
