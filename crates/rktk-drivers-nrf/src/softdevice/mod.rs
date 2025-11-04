//! Drivers that needs softdevice instance.

use core::mem;

use nrf_softdevice::{SocEvent, Softdevice, raw};
use rktk::utils::Channel;

#[cfg(feature = "softdevice-ble")]
pub mod ble;
pub mod flash;
#[cfg(feature = "softdevice-ble")]
pub mod split;
pub mod vbus;

/// Initialize the softdevice and return the instance.
///
/// # Usage
/// This function enables softdevice, but doesn't start. To start softdevice, call `start_softdevice`.
///
/// ```
/// let sd = init_softdevice(...);  // <-- Get mutable reference to softdevice
/// start_server(sd).await;         // <-- Use softdevice for function which requires mutable reference to softdevice.
/// start_softdevice(sd).await;     // <-- Starts softdevice. This function borrows softdevice forever, so from this point, you can only use immutable reference to softdevice.
/// get_flash(sd).await;            // <-- get_flash does not require mutable reference to softdevice, so you can use this after starting softdevice;
/// ```
#[allow(clippy::mut_from_ref)]
pub fn init_softdevice(ble_gap_name: &'static str) -> &'static mut Softdevice {
    let config = nrf_softdevice::Config {
        clock: Some(raw::nrf_clock_lf_cfg_t {
            source: raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 24,
        }),
        conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
            p_value: ble_gap_name.as_ptr() as _,
            current_len: 9,
            max_len: 9,
            write_perm: unsafe { mem::zeroed() },
            _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                raw::BLE_GATTS_VLOC_STACK as u8,
            ),
        }),
        conn_l2cap: Some(raw::ble_l2cap_conn_cfg_t {
            ch_count: 1,
            rx_mps: 247,
            tx_mps: 247,
            rx_queue_size: 10,
            tx_queue_size: 10,
        }),
        ..Default::default()
    };

    Softdevice::enable(&config)
}

/// Starts softdevice task
pub fn start_softdevice(spawner: embassy_executor::Spawner, sd: &'static Softdevice) {
    spawner.spawn(softdevice_task(sd)).unwrap();
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    unsafe {
        nrf_softdevice::raw::sd_power_usbpwrrdy_enable(1);
        nrf_softdevice::raw::sd_power_usbdetected_enable(1);
        nrf_softdevice::raw::sd_power_usbremoved_enable(1);
        nrf_softdevice::raw::sd_clock_hfclk_request();
    }

    sd.run_with_callback(|ev| {
        let _ = SD_SOCEVENT_CHAN.try_send(ev);
    })
    .await
}

pub type SdEventChan = Channel<SocEvent, 8>;
pub static SD_SOCEVENT_CHAN: SdEventChan = Channel::new();
