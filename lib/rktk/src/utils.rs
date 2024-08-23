#[macro_export]
macro_rules! print {
    ($literal:literal) => {{
        use $crate::task::display::*;
        let _ = DISPLAY_CONTROLLER.try_send(DisplayMessage::Message($literal));
    }};
    ($($arg:tt)*) => {{
        use $crate::task::display::*;
        use core::fmt::Write as _;

        let mut str = $crate::reexports::heapless::String::<256>::new();
        write!(str, $($arg)*).unwrap();
        let _ = DISPLAY_DYNAMIC_MESSAGE_CONTROLLER.try_send(str);
    }};
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        use core::fmt::Write as _;
        let mut str = $crate::reexports::heapless::String::<256>::new();
        write!(str, $($arg)*).unwrap();
        str
    }};
}

#[macro_export]
macro_rules! print_str {
    ($str:tt) => {{
        use $crate::task::display::*;
        let _ = DISPLAY_CONTROLLER.try_send(DisplayMessage::Message($str));
    }};
}

macro_rules! display_state {
    ($mes_type:ident,$val:expr) => {{
        use $crate::task::display::*;
        let _ = DISPLAY_CONTROLLER.try_send(DisplayMessage::$mes_type($val));
    }};
}
pub(crate) use display_state;
use embassy_sync::mutex::Mutex;

#[cfg(target_arch = "arm")]
pub(crate) type ThreadModeMutex<T> =
    Mutex<embassy_sync::blocking_mutex::raw::ThreadModeRawMutex, T>;
#[cfg(not(target_arch = "arm"))]
pub(crate) type ThreadModeMutex<T> =
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, T>;
