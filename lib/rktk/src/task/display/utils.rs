use embedded_graphics::{prelude::*, primitives::Rectangle};

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Rotate0,
    Rotate90,
}

#[derive(Debug)]
pub struct RotatedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    parent: &'a mut T,
    rotation: Rotation,
}

impl<'a, T> RotatedDrawTarget<'a, T>
where
    T: DrawTarget,
{
    pub fn new(parent: &'a mut T, rotation: Rotation) -> Self {
        Self { parent, rotation }
    }
}

impl<T> DrawTarget for RotatedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        match self.rotation {
            Rotation::Rotate0 => self.draw_iter(pixels),
            Rotation::Rotate90 => {
                let parent_height = self.parent.bounding_box().size.height as i32;

                self.parent.draw_iter(
                    pixels
                        .into_iter()
                        .map(|Pixel(p, c)| Pixel(Point::new(p.y, parent_height - p.x), c)),
                )
            }
        }
    }
}

impl<T> Dimensions for RotatedDrawTarget<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        match self.rotation {
            Rotation::Rotate0 => self.parent.bounding_box(),
            Rotation::Rotate90 => {
                let parent_bb = self.parent.bounding_box();
                Rectangle::new(
                    parent_bb.top_left,
                    Size::new(parent_bb.size.height, parent_bb.size.width),
                )
            }
        }
    }
}
