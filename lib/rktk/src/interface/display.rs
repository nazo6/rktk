use core::ops::DerefMut;

use display_interface::DisplayError;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embedded_graphics::{
    geometry::Point,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

const TEXT_STYLE: MonoTextStyle<'static, BinaryColor> = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .background_color(BinaryColor::Off)
    .build();

pub trait Display: DerefMut<Target = Self::DerefTarget> {
    type DerefTarget: DrawTarget<Color = BinaryColor>;

    fn flush(&mut self) -> Result<(), DisplayError>;
    fn clear_buffer(&mut self);
    async fn flush_async(&mut self) -> Result<(), DisplayError>;

    fn calculate_point(col: i32, row: i32) -> Point {
        Point::new((col - 1) * 6, (row - 1) * 10)
    }

    async fn clear(&mut self) -> Result<(), DisplayError> {
        self.clear_buffer();
        self.flush_async().await
    }

    async fn update_text(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, TEXT_STYLE, Baseline::Top).draw(self.deref_mut());
        self.flush_async().await
    }

    fn draw_text_blocking(&mut self, text: &str) -> Result<(), DisplayError> {
        self.clear_buffer();
        let _ = Text::with_baseline(text, Point::zero(), TEXT_STYLE, Baseline::Top)
            .draw(self.deref_mut());
        self.flush()
    }

    fn update_text_blocking(&mut self, text: &str, point: Point) -> Result<(), DisplayError> {
        let _ = Text::with_baseline(text, point, TEXT_STYLE, Baseline::Top).draw(self.deref_mut());
        self.flush()
    }
}

macro_rules! update_display {
    ($self:expr, $str:expr, $x:literal, $y:literal) => {
        let _ = $self
            .inner
            .lock()
            .await
            .as_mut()
            .unwrap()
            .update_text($str, D::calculate_point($x, $y))
            .await;
    };
}

#[derive(Default)]
pub struct GlobalDisplay<D: Display> {
    pub inner: Mutex<CriticalSectionRawMutex, Option<D>>,
}

impl<D: Display> GlobalDisplay<D> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    pub async fn init(&self, oled: D) {
        self.inner.lock().await.replace(oled);
    }

    pub fn try_draw_text(&self, str: &str) {
        if let Ok(mut display) = self.inner.try_lock() {
            let _ = display.as_mut().unwrap().draw_text_blocking(str);
        }
    }

    pub fn try_set_message(&self, str: &str) {
        if let Ok(mut display) = self.inner.try_lock() {
            let _ = display
                .as_mut()
                .unwrap()
                .update_text_blocking(str, D::calculate_point(1, 3));
        }
    }

    pub async fn set_message(&self, str: &str) {
        update_display!(self, "                    ", 1, 3);
        update_display!(self, str, 1, 3);
    }

    pub async fn set_master(&self, master: bool) {
        update_display!(self, if master { "M" } else { "S" }, 1, 1);
    }

    pub async fn set_mouse(&self, mouse: bool) {
        update_display!(self, if mouse { "m" } else { "x" }, 2, 1);
    }

    // pub async fn set_highest_layer(&self, layer: u8) {
    //     let mut str = heapless::String::<2>::new();
    //     write!(str, "{:1}", layer).unwrap();
    //     update_display!(self, &str, 5, 1);
    // }
    //
    // pub async fn set_mouse_pos(&self, x: i8, y: i8) {
    //     let mut str = heapless::String::<32>::new();
    //     write!(str, "[{:3},{:3}]", x, y).unwrap();
    //     update_display!(self, &str, 8, 1);
    // }

    pub fn inner(&self) -> &Mutex<CriticalSectionRawMutex, Option<D>> {
        &self.inner
    }
}
