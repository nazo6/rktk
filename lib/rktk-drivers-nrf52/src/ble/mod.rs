use core::mem;

use embassy_futures::select::select;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use nrf_softdevice::{
    ble::{
        advertisement_builder::{
            AdvertisementDataType, Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload,
            ServiceList, ServiceUuid16,
        },
        gatt_server, peripheral,
        security::SecurityHandler,
    },
    raw::{self},
    Softdevice,
};

use rktk::interface::{ble::BleDriver, usb::HidReport};
use server::Server;

mod constant;
mod server;
mod services;

static REPORT_CHAN: Channel<CriticalSectionRawMutex, HidReport, 8> = Channel::new();

pub struct NrfBleDriver {}

impl NrfBleDriver {
    pub async fn new_and_init(name: &'static str) -> Self {
        let spawner = embassy_executor::Spawner::for_current_executor().await;

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
                p_value: name.as_ptr() as _,
                current_len: 9,
                max_len: 9,
                write_perm: unsafe { mem::zeroed() },
                _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                    raw::BLE_GATTS_VLOC_STACK as u8,
                ),
            }),
            ..Default::default()
        };

        let sd: &'static mut Softdevice = Softdevice::enable(&config);

        unsafe {
            raw::sd_ble_gap_appearance_set(raw::BLE_APPEARANCE_HID_KEYBOARD as u16);
        }

        let server = server::Server::new(sd, "12345678").unwrap();
        spawner.spawn(softdevice_task(sd)).unwrap();
        spawner.spawn(server_task(sd, server, name)).unwrap();

        Self {}
    }
}

impl BleDriver for NrfBleDriver {
    async fn wait_ready(&mut self) {}

    async fn send_report(
        &mut self,
        report: rktk::interface::usb::HidReport,
    ) -> Result<(), rktk::interface::error::RktkError> {
        let _ = REPORT_CHAN.send(report).await;

        Ok(())
    }
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

struct HidSecurityHandler {}
impl SecurityHandler for HidSecurityHandler {
    fn can_bond(&self, _conn: &nrf_softdevice::ble::Connection) -> bool {
        true
    }
    fn on_bonded(
        &self,
        _conn: &nrf_softdevice::ble::Connection,
        _master_id: nrf_softdevice::ble::MasterId,
        _key: nrf_softdevice::ble::EncryptionInfo,
        _peer_id: nrf_softdevice::ble::IdentityKey,
    ) {
    }
}
static SEC: HidSecurityHandler = HidSecurityHandler {};

#[embassy_executor::task]
async fn server_task(sd: &'static Softdevice, server: Server, name: &'static str) -> ! {
    let adv_data: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(
            ServiceList::Complete,
            &[
                ServiceUuid16::DEVICE_INFORMATION,
                ServiceUuid16::BATTERY,
                ServiceUuid16::HUMAN_INTERFACE_DEVICE,
            ],
        )
        .full_name(name)
        // Change the appearance (icon of the bluetooth device) to a keyboard
        .raw(AdvertisementDataType::APPEARANCE, &[0xC1, 0x03])
        .raw(AdvertisementDataType::TXPOWER_LEVEL, &[0x02])
        .build();

    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new().build();

    let config = peripheral::Config::default();
    let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
        adv_data: &adv_data,
        scan_data: &SCAN_DATA,
    };

    loop {
        let mut cnt = 0;
        let conn = loop {
            match peripheral::advertise_pairable(sd, adv, &config, &SEC).await {
                Ok(conn) => break conn,
                Err(peripheral::AdvertiseError::Timeout) => {
                    rktk::print!("Timeout");
                    cnt += 1;
                    if cnt > 10 {
                        panic!("Failed to pair (10 tries)");
                    }
                }
                Err(e) => {
                    rktk::print!("Pair error: {:?}", e);
                    continue;
                }
            }
        };

        rktk::print!("Paired: {:X?}", conn.peer_address().bytes);

        select(
            async {
                let e = gatt_server::run(&conn, &server, |_| {}).await;
                rktk::print!("{:?}", e);
            },
            async {
                let mut start = embassy_time::Instant::now();
                let mut cnt = 0;
                loop {
                    let report = REPORT_CHAN.receive().await;
                    let _ = server.hid.send_report(&conn, report);
                    cnt += 1;

                    // if embassy_time::Instant::now() - start
                    //     > embassy_time::Duration::from_millis(1000)
                    // {
                    //     rktk::print!("Sent {} reports", cnt);
                    //     start = embassy_time::Instant::now();
                    //     cnt = 0;
                    // }
                }
            },
        )
        .await;

        rktk::print!("Disconnected");
    }
}
