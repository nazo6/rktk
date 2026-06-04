//! Generic MIPI display driver using `display-driver`

use display_interface::DisplayError;
use display_driver::{Area, FrameControl};
use rktk::drivers::interface::display::DisplayDriver;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::OriginDimensions,
    pixelcolor::{
        raw::{BigEndian, RawU16},
        Rgb565,
    },
    prelude::*,
    framebuffer::Framebuffer,
};

pub struct MipiDisplayWrapper<const W: usize, const H: usize, const SIZE: usize> {
    pub fb: Framebuffer<Rgb565, RawU16, BigEndian, W, H, SIZE>,
}

impl<const W: usize, const H: usize, const SIZE: usize> MipiDisplayWrapper<W, H, SIZE> {
    pub fn new() -> Self {
        Self {
            fb: Framebuffer::new(),
        }
    }
}

impl<const W: usize, const H: usize, const SIZE: usize> Default for MipiDisplayWrapper<W, H, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const W: usize, const H: usize, const SIZE: usize> DrawTarget for MipiDisplayWrapper<W, H, SIZE> {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.fb.draw_iter(pixels)
    }
}

impl<const W: usize, const H: usize, const SIZE: usize> OriginDimensions for MipiDisplayWrapper<W, H, SIZE> {
    fn size(&self) -> Size {
        Size::new(W as u32, H as u32)
    }
}

pub struct MipiDisplayDriver<BUS, PANEL, const W: usize, const H: usize, const SIZE: usize>
where
    BUS: display_driver::DisplayBus,
    PANEL: display_driver::Panel<BUS>,
{
    pub display: display_driver::DisplayDriver<BUS, PANEL>,
    pub wrapper: MipiDisplayWrapper<W, H, SIZE>,
}

impl<BUS, PANEL, const W: usize, const H: usize, const SIZE: usize> MipiDisplayDriver<BUS, PANEL, W, H, SIZE>
where
    BUS: display_driver::DisplayBus,
    PANEL: display_driver::Panel<BUS>,
{
    pub fn new(display: display_driver::DisplayDriver<BUS, PANEL>) -> Self {
        Self {
            display,
            wrapper: MipiDisplayWrapper::new(),
        }
    }
}

impl<BUS, PANEL, const W: usize, const H: usize, const SIZE: usize> DisplayDriver
    for MipiDisplayDriver<BUS, PANEL, W, H, SIZE>
where
    BUS: display_driver::DisplayBus + 'static,
    PANEL: display_driver::Panel<BUS> + 'static,
    BUS::Error: core::fmt::Debug + 'static,
    display_driver::DisplayError<BUS::Error>: From<BUS::Error>,
{
    type Color = Rgb565;
    type Display = MipiDisplayWrapper<W, H, SIZE>;

    fn draw_target(&mut self) -> &mut Self::Display {
        &mut self.wrapper
    }

    async fn init(&mut self) -> Result<(), DisplayError> {
        self.display.init(&mut embassy_time::Delay)
            .await
            .map_err(|_| DisplayError::BusWriteError)
    }

    async fn flush(&mut self) -> Result<(), DisplayError> {
        self.display.write_pixels(
            Area::new(0, 0, W as u16, H as u16),
            FrameControl::new_last(),
            self.wrapper.fb.data(),
        )
        .await
        .map_err(|_| DisplayError::BusWriteError)
    }

    async fn clear(&mut self) -> Result<(), DisplayError> {
        let _ = self.wrapper.fb.clear(Rgb565::BLACK);
        Ok(())
    }

    async fn set_display_on(&mut self, _on: bool) -> Result<(), DisplayError> {
        Ok(())
    }
}
