use core::marker::PhantomData;

use embassy_futures::join::join;
use embassy_sync::pipe::Pipe;
use nrf_softdevice::{
    ble::{
        advertisement_builder::{
            Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList,
        },
        l2cap, peripheral,
    },
    Softdevice,
};
use rktk::drivers::interface::split::SplitDriver;
use rktk::utils::RawMutex;
use rktk_log::error;

use super::packet::Packet;
use super::PSM;

static TX_PIPE: Pipe<RawMutex, 128> = Pipe::new();
static RX_PIPE: Pipe<RawMutex, 128> = Pipe::new();

#[embassy_executor::task]
async fn ble_split_peripheral_task(sd: &'static Softdevice) {
    embassy_time::Timer::after_secs(3).await;
    error!("ble split start");

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_128(ServiceList::Complete, &[super::RKTK_SPLIT_SERVICE_ID])
        .short_name("H")
        .build();

    static SCAN_DATA: [u8; 0] = [];

    let l = l2cap::L2cap::<Packet>::init(sd);

    let config = peripheral::Config::default();
    let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
        adv_data: &ADV_DATA,
        scan_data: &SCAN_DATA,
    };
    let conn = loop {
        match peripheral::advertise_connectable(sd, adv, &config).await {
            Ok(conn) => break conn,
            Err(e) => {
                rktk::print!("{:?}", e);
                continue;
            }
        };
    };

    rktk::print!("advertising done!");

    let config = l2cap::Config { credits: 8 };
    let ch = l.listen(&conn, &config, PSM).await.unwrap();
    rktk::print!("l2cap connected");

    let ch2 = ch.clone();
    join(
        async move {
            loop {
                let Ok(p) = ch.rx().await else {
                    continue;
                };
                rktk::print!("Received: {:?}", p);
                RX_PIPE.write_all(p.as_bytes()).await;
            }
        },
        async move {
            loop {
                let mut buf = [0; 128];
                let size = TX_PIPE.read(&mut buf).await;
                let packet = Packet::new(&buf[0..size]);
                rktk::print!("Sending: {:?}", packet);
                let _ = ch2.tx(packet).await;
            }
        },
    )
    .await;
}

/// Split driver for peripheral (slave) side.
pub struct SoftdeviceBlePeripheralSplitDriver {
    _phantom: PhantomData<()>,
}

impl SoftdeviceBlePeripheralSplitDriver {
    pub async fn new(sd: &'static Softdevice) -> Self {
        embassy_executor::Spawner::for_current_executor()
            .await
            .spawn(ble_split_peripheral_task(sd))
            .unwrap();

        Self {
            _phantom: PhantomData,
        }
    }
}

impl SplitDriver for SoftdeviceBlePeripheralSplitDriver {
    type Error = core::convert::Infallible;

    async fn recv(&mut self, buf: &mut [u8], _is_master: bool) -> Result<usize, Self::Error> {
        let size = RX_PIPE.read(buf).await;
        Ok(size)
    }

    async fn send_all(&mut self, buf: &[u8], _is_master: bool) -> Result<(), Self::Error> {
        TX_PIPE.write_all(buf).await;
        Ok(())
    }
}
