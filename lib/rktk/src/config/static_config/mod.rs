//! This module contains the static configuration of the firmware.
//! `static` means that the configuration is compiled into the firmware binary.
//!
//! # Configuration
//! Values in [`StaticConfig`] struct can be set from your keyboard crate through toml file. Here
//! is the way to set the configuration:
//!
//! 1. Set the `RKTK_CONFIG_PATH` environment variable.
//!    Easy way to set this value is to add following section to `.cargo/config.toml`:
//!    ```toml
//!    [env]
//!    RKTK_CONFIG_PATH = { value = "rktk.toml", relative = true }
//!    ```
//!    By adding this line, rktk will read `rktk.toml` file in the root of your keyboard crate.
//! 2. Create `rktk.toml` in the root of your keyboard crate.
//!    Here is an example of `rktk.toml`:
//!    ```toml
//!    "$schema" = "https://raw.githubusercontent.com/nazo6/rktk/master/lib/rktk/schema.json"
//!    
//!    cols = 7
//!    rows = 5
//!    
//!    right_led_count = 34
//!    left_led_count = 37
//!    ```
//!    Only required values are `cols` and `rows`. Other values are optional. We provide json
//!    schema so can be validated using [taplo](https://taplo.tamasfe.dev/).
//! 3. Run `cargo build`

mod schema;

use embassy_time::Duration;
pub use schema::StaticConfig;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

pub const SCAN_INTERVAL_KEYBOARD: Duration = Duration::from_millis(CONFIG.scan_interval_keyboard);
pub const SCAN_INTERVAL_MOUSE: Duration = Duration::from_millis(CONFIG.scan_interval_mouse);
