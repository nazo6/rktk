#[macro_export]
macro_rules! print {
    ($literal:literal) => {{
        use $crate::task::display::*;
        let _ = DISPLAY_CONTROLLER.try_send(DisplayMessage::Message($literal));
    }};
    ($($arg:tt)*) => {{
        use $crate::task::display::*;
        use core::fmt::Write as _;

        let mut str = heapless::String::<256>::new();
        write!(str, $($arg)*).unwrap();
        DISPLAY_DYNAMIC_MESSAGE_CONTROLLER.signal(str);
    }};
}

macro_rules! display_state {
    ($mes_type:ident,$val:expr) => {{
        use $crate::task::display::*;
        let _ = DISPLAY_CONTROLLER.try_send(DisplayMessage::$mes_type($val));
    }};
}
pub(crate) use display_state;
