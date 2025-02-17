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
    use core::future::Future;

    macro_rules! join_or_spawn {
        ($name:ident, $f1:ident: $f1t:ident, $($fs:ident: $fst:ident),* ) => {
            pub async fn $name<O, $f1t: Future<Output = O>, $($fst: Future + 'static),*>($f1: $f1t, $($fs: $fst),*) -> O {
                #[cfg(feature = "alloc")]
                {
                    use alloc::boxed::Box;

                    let ex = embassy_executor::Spawner::for_current_executor().await;
                    $(
                        let ts = Box::leak(Box::new(embassy_executor::raw::TaskStorage::new()));
                        let st = ts.spawn(|| $fs);
                        ex.spawn(st)
                            .unwrap();
                    )*
                    $f1.await
                }

                #[cfg(not(feature = "alloc"))]
                embassy_futures::join::$name($f1, $($fs),*).await.0
            }
        };
    }

    join_or_spawn!(join, f1: F1, f2: F2);
    join_or_spawn!(join3, f1: F1, f2: F2, f3: F3);
    join_or_spawn!(join4, f1: F1, f2: F2, f3: F3, f4: F4);
}
