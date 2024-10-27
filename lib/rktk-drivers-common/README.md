# rktk-drivers-common

`rktk-drivers-common` is a collection of drivers for rktk, which is common to
all platforms.

This library is the basis of `rktk-drivers-nrf` and `nrf-drivers-rp`, and by
utilizing traits defined by embassy etc., you can create drivers with only a
very thin wrapper for MCUs that have embassy hal.

Users who want to build keyboard should use mcu-specific crate like
`rktk-drivers-nrf` instead of this crate.
