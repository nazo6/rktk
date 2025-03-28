use embassy_futures::{join::join, select::select};
use rktk::{drivers::interface::BackgroundTask, utils::Receiver};
use rktk_log::{helper::Debug2Format, info, warn};
use ssmarshal::serialize;
use trouble_host::{
    Address, Controller, Error, Host, HostResources,
    gap::{GapConfig, PeripheralConfig},
    prelude::*,
};
use usbd_hid::descriptor::KeyboardReport;

use super::server::Server;

pub struct TroubleReporterTask<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> {
    pub(super) controller: C,
    pub(super) output_rx: Receiver<'static, KeyboardReport, 4>,
}

impl<
    C: Controller + 'static,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
> BackgroundTask for TroubleReporterTask<C, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU>
{
    async fn run(self) {
        rktk::print!("BLE start");
        let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
        info!("Our address = {:?}", address);

        let mut resources: HostResources<CONNECTIONS_MAX, L2CAP_CHANNELS_MAX, L2CAP_MTU> =
            HostResources::new();
        let stack = trouble_host::new(self.controller, &mut resources).set_random_address(address);
        let Host {
            mut peripheral,
            runner,
            ..
        } = stack.build();

        info!("Starting advertising and GATT service");
        let server = Server::new_with_config(GapConfig::Peripheral(PeripheralConfig {
            name: "TrouBLE",
            appearance: &appearance::power_device::GENERIC_POWER_DEVICE,
        }));
        match server {
            Ok(server) => {
                let _ = join(ble_task(runner), async {
                    loop {
                        match advertise("Trouble Example", &mut peripheral).await {
                            Ok(conn) => {
                                let gatt_conn = conn.with_attribute_server(&server).unwrap();
                                let a = gatt_events_task(&gatt_conn);
                                let b = hid_task(&server, &gatt_conn, &stack, &self.output_rx);
                                // run until any task ends (usually because the connection has been closed),
                                // then return to advertising state.
                                select(a, b).await;
                            }
                            Err(e) => {
                                #[cfg(feature = "defmt")]
                                let e = defmt::Debug2Format(&e);
                                rktk_log::error!("[adv] error: {:?}", e);
                                return;
                            }
                        }
                    }
                })
                .await;
            }
            Err(e) => {
                rktk_log::error!("[gatt] error: {:?}", e);
            }
        }
    }
}

async fn ble_task<C: Controller>(mut runner: Runner<'_, C>) {
    loop {
        if let Err(e) = runner.run().await {
            #[cfg(feature = "defmt")]
            let e = defmt::Debug2Format(&e);
            rktk_log::error!("{:?}", e);
            return;
        }
    }
}

async fn gatt_events_task(conn: &GattConnection<'_, '_>) -> Result<(), Error> {
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => {
                match event {
                    // Server processing emits
                    Ok(event) => match event.accept() {
                        Ok(reply) => {
                            reply.send().await;
                        }
                        Err(e) => {
                            warn!("[gatt] error sending response: {:?}", e);
                        }
                    },
                    Err(e) => {
                        warn!("[gatt] error processing event: {:?}", e);
                    }
                }
            }
            _ => {}
        }
    }
    info!("[gatt] task finished");
    Ok(())
}

/// Create an advertiser to use to connect to a BLE Central, and wait for it to connect.
async fn advertise<'a, C: Controller>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C>,
) -> Result<Connection<'a>, BleHostError<C::Error>> {
    let mut advertiser_data = [0; 31];
    AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ServiceUuids16(&[[0x0f, 0x18], [0x0a, 0x18], [0x12, 0x18]]),
            AdStructure::CompleteLocalName(name.as_bytes()),
        ],
        &mut advertiser_data[..],
    )?;
    let advertiser = peripheral
        .advertise(
            &Default::default(),
            Advertisement::ConnectableScannableUndirected {
                adv_data: &advertiser_data[..],
                scan_data: &[],
            },
        )
        .await?;
    info!("[adv] advertising");
    let conn = advertiser.accept().await?;
    info!("[adv] connection established");
    Ok(conn)
}

async fn hid_task<C: Controller>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_>,
    _stack: &Stack<'_, C>,
    output_rx: &Receiver<'static, KeyboardReport, 4>,
) {
    loop {
        let report = output_rx.receive().await;
        rktk_log::info!("got report to send");

        let mut buf = [0u8; 8];
        match serialize(&mut buf, &report) {
            Ok(n) => match server.hid_service.output_keyboard.notify(&conn, &buf).await {
                Ok(_) => {
                    rktk_log::info!("sent report");
                }
                Err(e) => {
                    rktk_log::error!("failed to send report: {:?}", e);
                }
            },
            Err(e) => {
                rktk_log::error!("failed to serialize report: {:?}", Debug2Format(&e));
            }
        }
    }
}
