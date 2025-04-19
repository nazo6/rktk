use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
};

// 'usb', 16x16px
pub const IMAGE_USB: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0x80, 0x19, 0x9c,
            0x19, 0xbc, 0x19, 0x9c, 0x19, 0x98, 0x19, 0xb8, 0x1f, 0xf0, 0x07, 0x80, 0x01, 0x80,
            0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00,
        ],
        16,
    ),
    Point::zero(),
);

// 'bluetooth-x', 16x16px
pub const IMAGE_BLUETOOTH: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x01, 0xc0, 0x01, 0xe0, 0x05, 0xb0, 0x07, 0xe0,
            0x03, 0xc0, 0x03, 0xc0, 0x07, 0xe0, 0x05, 0xb0, 0x01, 0xe0, 0x01, 0xc0, 0x01, 0x80,
            0x00, 0x00, 0x00, 0x00,
        ],
        16,
    ),
    Point::zero(),
);

pub const IMAGE_MOUSE: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x07, 0x80, 0x1b, 0x60, 0x13, 0x20, 0x13, 0x20, 0x1f, 0xe0, 0x13, 0x20,
            0x10, 0x20, 0x10, 0x20, 0x10, 0x20, 0x10, 0x20, 0x18, 0x60, 0x07, 0x80, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
        14,
    ),
    Point::zero(),
);

/*
// 'battery-0', 24x18px
pub const IMAGE_BATTERY_0: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xff, 0xc0, 0x0f, 0xff,
            0xe0, 0x18, 0x00, 0x30, 0x18, 0x00, 0x30, 0x18, 0x00, 0x18, 0x18, 0x00, 0x18, 0x18,
            0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x30, 0x18, 0x00, 0x30, 0x0f, 0xff, 0xe0,
            0x07, 0xff, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        20,
    ),
    Point::zero(),
);

// 'battery-1', 24x18px
pub const IMAGE_BATTERY_1: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xff, 0xc0, 0x0f, 0xff,
            0xe0, 0x18, 0x00, 0x30, 0x1b, 0x00, 0x30, 0x1b, 0x00, 0x18, 0x1b, 0x00, 0x18, 0x1b,
            0x00, 0x18, 0x1b, 0x00, 0x18, 0x1b, 0x00, 0x30, 0x18, 0x00, 0x30, 0x0f, 0xff, 0xe0,
            0x07, 0xff, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        20,
    ),
    Point::zero(),
);
// 'battery-2', 24x18px
pub const IMAGE_BATTERY_2: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xff, 0xc0, 0x0f, 0xff,
            0xe0, 0x18, 0x00, 0x30, 0x1b, 0x60, 0x30, 0x1b, 0x60, 0x18, 0x1b, 0x60, 0x18, 0x1b,
            0x60, 0x18, 0x1b, 0x60, 0x18, 0x1b, 0x60, 0x30, 0x18, 0x00, 0x30, 0x0f, 0xff, 0xe0,
            0x07, 0xff, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        20,
    ),
    Point::zero(),
);
// 'battery-3', 24x18px
pub const IMAGE_BATTERY_3: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xff, 0xc0, 0x0f, 0xff,
            0xe0, 0x18, 0x00, 0x30, 0x1b, 0x6c, 0x30, 0x1b, 0x6c, 0x18, 0x1b, 0x6c, 0x18, 0x1b,
            0x6c, 0x18, 0x1b, 0x6c, 0x18, 0x1b, 0x6c, 0x30, 0x18, 0x00, 0x30, 0x0f, 0xff, 0xe0,
            0x07, 0xff, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        20,
    ),
    Point::zero(),
);
// 'battery-4', 24x18px
pub const IMAGE_BATTERY_4: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xff, 0xc0, 0x0f, 0xff,
            0xe0, 0x18, 0x00, 0x30, 0x1b, 0x6d, 0xb0, 0x1b, 0x6d, 0x98, 0x1b, 0x6d, 0x98, 0x1b,
            0x6d, 0x98, 0x1b, 0x6d, 0x98, 0x1b, 0x6d, 0xb0, 0x18, 0x00, 0x30, 0x0f, 0xff, 0xe0,
            0x07, 0xff, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        20,
    ),
    Point::zero(),
);
// 'battery-charging', 24x18px
pub const IMAGE_BATTERY_CHARGING: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x81, 0xc0, 0x0f, 0x99,
            0xe0, 0x18, 0x18, 0x30, 0x18, 0x30, 0x30, 0x18, 0x30, 0x18, 0x18, 0x7c, 0x18, 0x18,
            0x7c, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x30, 0x18, 0x30, 0x30, 0x0f, 0x33, 0xe0,
            0x07, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ],
        20,
    ),
    Point::zero(),
);

// 'bluetooth_x', 16x16px
pub const IMAGE_BLUETOOTH_X: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x01, 0xa4, 0x01, 0xbc, 0x05, 0x98, 0x07, 0xbc,
            0x03, 0x80, 0x03, 0xc0, 0x07, 0xe0, 0x05, 0xb0, 0x01, 0xe0, 0x01, 0xc0, 0x01, 0x80,
            0x00, 0x00, 0x00, 0x00,
        ],
        16,
    ),
    Point::zero(),
);
*/
