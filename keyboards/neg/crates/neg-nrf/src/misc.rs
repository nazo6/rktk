use rktk::{
    config::{Hand, RktkOpts, keymap::Keymap},
    task::display::default_display::DefaultDisplayConfig,
};

#[cfg(feature = "paw3395")]
use rktk_drivers_common::mouse::paw3395;
#[cfg(feature = "paw3395")]
pub const PAW3395_CONFIG: paw3395::config::Config = paw3395::config::Config {
    mode: paw3395::config::HP_MODE,
    lift_cutoff: paw3395::config::LiftCutoff::_2mm,
};

pub fn translate_key_position(row: usize, col: usize) -> Option<(usize, usize)> {
    #[cfg(feature = "left")]
    {
        Some((row, 7 - col))
    }

    #[cfg(feature = "right")]
    {
        Some((row, col))
    }
}

pub fn get_opts(keymap: &'static Keymap) -> RktkOpts<DefaultDisplayConfig, Layout> {
    RktkOpts {
        keymap,
        hand: Some(HAND),
        config: &rktk::config::DYNAMIC_CONFIG_FROM_FILE,
        display: DefaultDisplayConfig,
        rgb_layout: Layout,
    }
}

pub const HAND: Hand = {
    #[cfg(feature = "left")]
    {
        Hand::Left
    }
    #[cfg(feature = "right")]
    {
        Hand::Right
    }
};

#[cfg(feature = "left")]
pub use layout::LayoutLeft as Layout;

#[cfg(feature = "right")]
pub use layout::LayoutRight as Layout;

mod layout {
    use rktk::config::rgb::{layout::*, layout2d};

    // Right half RGB layout:
    // h      +.1 +.2 +.3 +.4 +.5 +.6 +.7 +.8
    // v-------------------------------------
    // +1.0 |           4   8  12  16  20  25
    // +0.5 |           5   9  13  17  21  26
    //  0.0 |           6  10  14  18  22  27
    // -0.5 |       2   7  11  15  19  23  28
    // -1.0 |   1   3                  24  29
    layout2d!(
        pub LayoutRight,
        [
            Shape2d::Point(Vec2::new(0.1, -1.0)),
            Shape2d::Point(Vec2::new(0.2, -0.5)),
            Shape2d::Point(Vec2::new(0.2, -1.0)),
            Shape2d::Grid {
                start: Vec2::new(0.3, 1.0),
                vertical_end: Vec2::new(0.6, 1.0),
                horizontal_end: Vec2::new(0.3, -0.5),
                horizontal_pixel_count: 4,
                vertical_pixel_count: 4,
                serpentine: true,
            },
            Shape2d::Grid {
                start: Vec2::new(0.7, 1.0),
                vertical_end: Vec2::new(0.8, 1.0),
                horizontal_end: Vec2::new(0.7,-1.0),
                horizontal_pixel_count: 5,
                vertical_pixel_count: 2,
                serpentine: true,
            }
        ]
    );

    // Left half RGB layout:
    // h      -.8 -.7 -.6 -.5 -.4 -.3 -.2 -.1
    // v-------------------------------------
    // +1.0 |  33  28  23  18  13  8   3
    // +0.5 |  34  29  24  19  14  9   4
    //  0.0 |  35  30  25  20  15  10  5
    // -0.5 |  36  31  26  21  16  11  6   1
    // -1.0 |  37  32  27  22  17  12  7   2
    layout2d!(
        pub LayoutLeft,
        [
            Shape2d::Point(Vec2::new(-0.1, -0.5)),
            Shape2d::Point(Vec2::new(-0.1, -1.0)),
            Shape2d::Grid {
                start: Vec2::new(-0.2, 1.0),
                vertical_end: Vec2::new(-0.8, 1.0),
                horizontal_end: Vec2::new(-0.2, -1.0),
                horizontal_pixel_count: 7,
                vertical_pixel_count: 5,
                serpentine: true,
            },
        ]
    );
}
