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
pub mod sjoin {
    #[macro_export]
    macro_rules! join {
        ($f1:expr, $($future:expr),* ) => {
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

                $f1.await
            };

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
