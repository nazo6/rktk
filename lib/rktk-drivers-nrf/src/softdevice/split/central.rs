use core::marker::PhantomData;
use core::slice;

use embassy_futures::join::join;
use embassy_sync::pipe::Pipe;
use nrf_softdevice::ble::{central, l2cap, Address, TxPower};
use nrf_softdevice::Softdevice;
use rktk::drivers::interface::split::SplitDriver;
use rktk::utils::RawMutex;
use rktk_log::{error, info};

use super::packet::Packet;
use super::{PSM, RKTK_SPLIT_SERVICE_ID};

static TX_PIPE: Pipe<RawMutex, 128> = Pipe::new();
static RX_PIPE: Pipe<RawMutex, 128> = Pipe::new();

#[embassy_executor::task]
async fn ble_split_central_task(sd: &'static Softdevice) {
    embassy_time::Timer::after_secs(3).await;
    error!("Scanning for peer...");

    let config = central::ScanConfig {
        whitelist: None,
        tx_power: TxPower::ZerodBm,
        ..Default::default()
    };
    let res = central::scan(sd, &config, |params| {
        let mut data =
            unsafe { slice::from_raw_parts(params.data.p_data, params.data.len as usize) };
        while !data.is_empty() {
            let len = data[0] as usize;
            if data.len() < len + 1 {
                break;
            }
            if len < 1 {
                break;
            }
            let key = data[1];
            let value = &data[2..len + 1];

            if value == RKTK_SPLIT_SERVICE_ID {
                // info!("{:X}:{:X?}", key, &value);
                return Some(Address::from_raw(params.peer_addr));
            }
            data = &data[len + 1..];
        }
        None
    })
    .await;

    let address = res.unwrap();
    rktk::print!("Found slave address {:?}", address);

    let addrs = &[&address];

    let mut config = central::ConnectConfig::default();
    config.scan_config.whitelist = Some(addrs);
    let conn = match central::connect(sd, &config).await {
        Ok(conn) => conn,
        Err(e) => {
            error!("connect failed: {:?}", e);
            return;
        }
    };

    let l = l2cap::L2cap::<super::packet::Packet>::init(sd);
    let config = l2cap::Config { credits: 8 };
    let ch = match l.setup(&conn, &config, PSM).await {
        Ok(ch) => ch,
        Err(e) => {
            error!("l2cap connect failed: {:?}", e);
            return;
        }
    };

    info!("Starting split handler");

    let ch2 = ch.clone();
    join(
        async move {
            loop {
                let Ok(p) = ch.rx().await else {
                    embassy_time::Timer::after_millis(100).await;
                    continue;
                };
                RX_PIPE.write_all(p.as_bytes()).await;
            }
        },
        async move {
            loop {
                let mut buf = [0; 128];
                let size = TX_PIPE.read(&mut buf).await;
                let packet = Packet::new(&buf[0..size]);
                let _ = ch2.tx(packet).await;
            }
        },
    )
    .await;
}

/// Split driver for central (master) side.
pub struct SoftdeviceBleCentralSplitDriver {
    _phantom: PhantomData<()>,
}

impl SoftdeviceBleCentralSplitDriver {
    pub async fn new(sd: &'static Softdevice) -> Self {
        embassy_executor::Spawner::for_current_executor()
            .await
            .spawn(ble_split_central_task(sd))
            .unwrap();

        Self {
            _phantom: PhantomData,
        }
    }
}

impl SplitDriver for SoftdeviceBleCentralSplitDriver {
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
