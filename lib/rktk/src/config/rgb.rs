pub use blinksy::layout;
pub use blinksy::layout2d;

pub struct DummyLayout;

impl layout::Layout2d for DummyLayout {
    const PIXEL_COUNT: usize = 0;

    fn shapes() -> impl Iterator<Item = layout::Shape2d> {
        [].into_iter()
    }
}
