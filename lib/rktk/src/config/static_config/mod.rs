//! This module contains the static configuration of the firmware.
//! `static` means that the configuration is compiled into the firmware binary.
//!
//! # Configuration
//! Values in [`StaticConfig`] struct can be set from your keyboard crate through json file. Here
//! is the way to set the configuration:
//!
//! 1. Set the `RKTK_CONFIG_PATH` environment variable.
//!    Easy way to set this value is to add following section to `.cargo/config.toml`:
//!    ```toml
//!    [env]
//!    RKTK_CONFIG_PATH = { value = "rktk.json", relative = true }
//!    ```
//!    By adding this line, rktk will read `rktk.json` file in the root of your keyboard crate.
//! 2. Create `rktk.json` in the root of your keyboard crate.
//!    Here is an example of `rktk.json`:
//!    ```json
//!    {
//!      "$schema": "https://raw.githubusercontent.com/nazo6/rktk/master/lib/rktk/schema.json",
//!      "keyboard": {
//!        "cols": 14,
//!        "rows": 5,
//!    
//!        "right_led_count": 34,
//!        "left_led_count": 37,
//!    
//!        "name": "keyboard name",
//!        "layout": <Put layout data of kle here>
//!      },
//!      "config": {
//!        "rktk": {
//!          "scan_interval_keyboard": 10,
//!          "scan_interval_mouse": 5
//!        }
//!      }
//!    }
//!
//!    ```
//! 3. Run `cargo build`

mod schema;

use embassy_time::Duration;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

pub const KEYBOARD: schema::Keyboard = CONFIG.keyboard;
pub(crate) const RKTK_CONFIG: schema::RktkConfig = CONFIG.config.rktk;

pub const SCAN_INTERVAL_KEYBOARD: Duration =
    Duration::from_millis(RKTK_CONFIG.scan_interval_keyboard);
pub const SCAN_INTERVAL_MOUSE: Duration = Duration::from_millis(RKTK_CONFIG.scan_interval_mouse);
