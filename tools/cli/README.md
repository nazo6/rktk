# rktk-cli

`rktk-cli` is a tool to help building firmware using rktk. It's basically just a
wrapper around cargo build, but it helps you properly target, profile and create
more optimized binaries, generate uf2 files, etc.

**NOTE:** This tool requires _nightly_ toolchain because it uses cargo's nightly
options like "rustflags in profile" to generate highly size-optimized binary.

For detail about rktk, see [RKTK project README](https://github.com/nazo6/rktk)

## Installation

```sh
cargo install rktk-cli
```

## Usage

See `rktk-cli build --help` for detail.

## Configuration

To use `rktk-cli build`, you have to specify mcu to build. This can be specified
through command line option or configuration in `Cargo.toml`'s
`package.metadata.rktk-cli` section.

Here is the example configuration.

```toml:Cargo.toml
[package.metadata.rktk-cli]
# Required. You can choose `Rp2040` or `Nrf52840`.
mcu = "Rp2040"

# Optional. By default, "MinSize" profile will be selected.
profile = "min-size"
```
