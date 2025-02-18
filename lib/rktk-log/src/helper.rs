pub trait FormatOrDebug {}

#[derive(Debug)]
pub struct Debug2Format<'a, T: core::fmt::Debug + ?Sized>(pub &'a T);

#[cfg(feature = "defmt")]
impl<T: core::fmt::Debug + ?Sized> defmt::Format for Debug2Format<'_, T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::Debug2Format(self.0).format(f)
    }
}

pub struct Display2Format<'a, T: core::fmt::Display + ?Sized>(pub &'a T);

impl<T: core::fmt::Display + ?Sized> core::fmt::Display for Display2Format<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "defmt")]
impl<T: core::fmt::Display + ?Sized> defmt::Format for Display2Format<'_, T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::Display2Format(self.0).format(f)
    }
}
