/// Convenient macro to obtain proper VBUS detection implementation
///
/// VBUS detection is a mechanism for detecting when USB is connected.
/// It is used to determine which side will take the master role in a split keyboard,
/// and is also required when building USB drivers.
///
/// In embassy-nrf, VBUS detection can be performed using `HardwareVbusDetect`, but `HardwareVbusDetect` cannot be used when using a SoftDevice, and a different mechanism must be used.
///
/// This macro therefore returns an appropriate VbusDetect implementation depending on whether the `softdevice` feature is present or not.
///
/// For more advanced use (such as when you want to handle SoftDevice SocEvents yourself), see [`crate::softdevice::vbus`].
///
/// **WARN: Calling this macro more than twice may cause a panic.**
#[cfg(feature = "softdevice")]
#[macro_export]
macro_rules! get_vbus {
    ($spawner:expr, $irqs:expr) => {{
        use $crate::softdevice::{SD_SOCEVENT_SIGNAL, vbus::SoftdeviceVbusDetect};

        SoftdeviceVbusDetect::init($spawner, &SD_SOCEVENT_SIGNAL)
    }};
}

#[cfg(not(feature = "softdevice"))]
#[macro_export]
macro_rules! get_vbus {
    ($spawner:expr, $irqs:expr) => {{
        use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;

        HardwareVbusDetect::new($irqs)
    }};
}
