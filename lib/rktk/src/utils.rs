//! Util types

/// Print to the display
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
        let _ = DISPLAY_DYNAMIC_MESSAGE_CONTROLLER.signal(str);
    }};
}

/// Print to the display without formatting.
///
/// For string that does not require formatting, this macro is more efficient than `print!`.
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

// Usually, we don't touch the interrupt code directly, so ThreadModeRawMutex is enough.
#[cfg(target_arch = "arm")]
pub type RawMutex = embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
#[cfg(not(target_arch = "arm"))]
pub type RawMutex = embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

pub type Mutex<T> = embassy_sync::mutex::Mutex<RawMutex, T>;
pub type Channel<T, const N: usize> = embassy_sync::channel::Channel<RawMutex, T, N>;
pub type Sender<'a, T, const N: usize> = embassy_sync::channel::Sender<'a, RawMutex, T, N>;
pub type Receiver<'a, T, const N: usize> = embassy_sync::channel::Receiver<'a, RawMutex, T, N>;
pub type Signal<T> = embassy_sync::signal::Signal<RawMutex, T>;

/// sjoin or "spawn or join"
pub(crate) mod sjoin {
    macro_rules! join {
        (async move $f1:expr, $($future:expr),* ) => {
            let _async_move = ();

            $crate::utils::sjoin::join!(@alloc {$f1}, $($future),* );
            $crate::utils::sjoin::join!(@no_alloc async move {$f1}, $($future),* );
        };
        (async $f1:expr, $($future:expr),* ) => {
            let _async = ();
            $crate::utils::sjoin::join!(@alloc {$f1}, $($future),* );
            $crate::utils::sjoin::join!(@no_alloc async {$f1}, $($future),* );
        };
        (@alloc $f1:expr, $($future:expr),* ) => {
            #[cfg(feature = "alloc")]
            {
                use alloc::boxed::Box;
                {
                    let ex = embassy_executor::Spawner::for_current_executor().await;
                    {
                        $(
                            let ts = Box::leak(Box::new(embassy_executor::raw::TaskStorage::new()));
                            let st = ts.spawn(|| $future);
                            ex.spawn(st).unwrap();
                        )*
                    }
                }

                $f1
            };
        };
        (@no_alloc $f1:expr, $($future:expr),* ) => {
            #[cfg(not(feature = "alloc"))]
            futures::join!($f1, $($future),* );
        };
    }
    pub(crate) use join;
}

#[macro_export]
macro_rules! singleton {
    ($val:expr, $type:ty) => {{
        static STATIC_CELL: $crate::reexports::static_cell::StaticCell<$type> =
            $crate::reexports::static_cell::StaticCell::new();
        STATIC_CELL.init($val)
    }};
}
