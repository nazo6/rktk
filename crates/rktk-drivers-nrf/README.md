# rktk-driver-nrf

rktk drivers for nrf chip

For more detail, see [RKTK project README](https://github.com/nazo6/rktk)

## Dependencies

'libclang' must be installed to use `sdc` feature.

## Note about `sdc` feature

The `sdc` feature is not available in the crates.io version of rktk-drivers-nrf.
It fails to compile when the feature is enabled. If you want to use the `sdc`
feature, use the git version of this crate.
