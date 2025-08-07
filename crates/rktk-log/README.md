# rktk-log

logger for rktk

## Credits

- Most code are from [defmt-or-log](https://github.com/t-moe/defmt-or-log).

## About

crate which uses macro in this crate must add below deps and features to
Cargo.toml to work correctly.

```toml
[dependencies]
rktk-log = { version = "" }
log = { version="", optional = true }
defmt = { version="", optional = true }

[features]
defmt = ["dep:defmt", "rktk-log/defmt"]
log = ["dep:log", "rktk-log/log"]
```

Also you should consider to modify defmt to feature if your depedency provides
feature for defmt.

## Viewing defmt-usb log

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
