use embassy_futures::select::select;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use nrf_softdevice::{
    ble::{
        advertisement_builder::{
            AdvertisementDataType, Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload,
            ServiceList, ServiceUuid16,
        },
        gatt_server, peripheral,
    },
    raw::{self},
    Softdevice,
};

use rktk::interface::{ble::BleDriver, error::RktkError, reporter::ReporterDriver};
use server::Server;
use usbd_hid::descriptor::{KeyboardReport, MediaKeyboardReport, MouseReport};

use super::flash::NrfDb;

mod bonder;
mod constant;
mod server;
mod services;

#[derive(Debug)]
pub enum HidReport {
    Keyboard(KeyboardReport),
    MediaKeyboard(MediaKeyboardReport),
    Mouse(MouseReport),
}

static REPORT_CHAN: Channel<CriticalSectionRawMutex, HidReport, 8> = Channel::new();

pub struct NrfBleDriver {}

pub async fn init_ble_server(sd: &'static mut Softdevice) -> (Server, &'static mut Softdevice) {
    unsafe {
        raw::sd_ble_gap_appearance_set(raw::BLE_APPEARANCE_HID_KEYBOARD as u16);
    }

    (server::Server::new(sd, "12345678").unwrap(), sd)
}

impl NrfBleDriver {
    pub async fn new(
        sd: &'static Softdevice,
        server: Server,
        name: &'static str,
        flash: &'static NrfDb,
    ) -> Self {
        let spawner = embassy_executor::Spawner::for_current_executor().await;
        spawner.spawn(server_task(sd, server, name, flash)).unwrap();
        Self {}
    }
}

impl ReporterDriver for NrfBleDriver {
    fn send_keyboard_report(&self, report: KeyboardReport) -> Result<(), RktkError> {
        REPORT_CHAN
            .try_send(HidReport::Keyboard(report))
            .map_err(|_| RktkError::GeneralError("report_chan not empty"))?;
        Ok(())
    }

    fn send_media_keyboard_report(&self, report: MediaKeyboardReport) -> Result<(), RktkError> {
        REPORT_CHAN
            .try_send(HidReport::MediaKeyboard(report))
            .map_err(|_| RktkError::GeneralError("report_chan not empty"))?;
        Ok(())
    }

    fn send_mouse_report(&self, report: MouseReport) -> Result<(), RktkError> {
        REPORT_CHAN
            .try_send(HidReport::Mouse(report))
            .map_err(|_| RktkError::GeneralError("report_chan not empty"))?;
        Ok(())
    }

    fn send_rrp_data(&self, _data: &[u8]) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }

    async fn read_rrp_data(&self, _buf: &mut [u8]) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }

    fn wakeup(&mut self) -> Result<(), RktkError> {
        Err(RktkError::NotSupported)
    }
}
impl BleDriver for NrfBleDriver {}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::task]
async fn server_task(
    sd: &'static Softdevice,
    server: Server,
    name: &'static str,
    db: &'static NrfDb,
) -> ! {
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

    let bonder = bonder::init_bonder();

    loop {
        let mut cnt = 0;
        let conn = loop {
            match peripheral::advertise_pairable(sd, adv, &config, bonder).await {
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
                loop {
                    let report = REPORT_CHAN.receive().await;
                    let _ = server.hid.send_report(&conn, report);
                }
            },
        )
        .await;

        rktk::print!("Disconnected");
    }
}
