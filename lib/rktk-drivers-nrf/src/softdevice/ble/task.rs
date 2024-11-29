use embassy_futures::select::select;
use nrf_softdevice::ble::{
    advertisement_builder::{
        AdvertisementDataType, Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload,
        ServiceList, ServiceUuid16,
    },
    gatt_server, peripheral,
};
use nrf_softdevice::Softdevice;
use rktk::interface::BackgroundTask;

use crate::softdevice::ble::REPORT_CHAN;
use crate::softdevice::flash::SharedFlash;

use super::server::Server;

pub struct SoftdeviceBleTask {
    pub sd: &'static Softdevice,
    pub server: Server,
    pub name: &'static str,
    pub flash: &'static SharedFlash,
}

impl BackgroundTask for SoftdeviceBleTask {
    async fn run(self) {
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
            .full_name(self.name)
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

        let bonder = super::bonder::init_bonder(self.flash).await;

        loop {
            rktk::print!("Advertising");

            let mut cnt = 0;
            let conn = loop {
                match peripheral::advertise_pairable(self.sd, adv, &config, bonder).await {
                    Ok(conn) => break conn,
                    Err(peripheral::AdvertiseError::Timeout) => {
                        cnt += 1;
                        if cnt > 10 {
                            rktk::print!("Failed to pair (10 tries)");
                        }
                    }
                    Err(e) => {
                        rktk::print!("Pair error: {:?}", e);
                        continue;
                    }
                }
            };

            // rktk::print!("Connected: {:X?}", conn.peer_address().bytes);

            select(
                async {
                    let e = gatt_server::run(&conn, &self.server, |_| {}).await;
                    log::info!("Server exited: {:?}", e);
                },
                async {
                    loop {
                        let report = REPORT_CHAN.receive().await;
                        if let Err(e) = self.server.hid.send_report(&conn, report) {
                            log::warn!("BLE hid failed: {:?}", e);
                        };
                    }
                },
            )
            .await;

            rktk::print!("Disconnected");
        }
    }
}
