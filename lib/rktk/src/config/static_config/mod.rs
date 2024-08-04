mod schema;

use embassy_time::Duration;
pub use schema::StaticConfig;

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

pub const SCAN_INTERVAL_KEYBOARD: Duration = Duration::from_millis(CONFIG.scan_interval_keyboard);
pub const SCAN_INTERVAL_MOUSE: Duration = Duration::from_millis(CONFIG.scan_interval_mouse);
