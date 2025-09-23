use embassy_futures::{join::join, select::select};
use rand_core::{CryptoRng, RngCore};
use rktk::utils::Receiver;
use rktk_log::{info, warn};
use ssmarshal::serialize;
use trouble_host::{
    Address, Controller, Error, Host, HostResources,
    gap::{GapConfig, PeripheralConfig},
    prelude::*,
};

use super::{Report, TroubleReporterConfig, server::Server};

pub async fn run<
    C: Controller + 'static,
    RNG: RngCore + CryptoRng,
    const CONNECTIONS_MAX: usize,
    const L2CAP_CHANNELS_MAX: usize,
    const L2CAP_MTU: usize,
>(
    controller: C,
    rng: &mut RNG,
    output_rx: Receiver<'static, Report, 4>,
    config: TroubleReporterConfig,
) {
    info!("Trouble BLE starting");
    let address: Address = Address::random([0xff, 0x8f, 0x1a, 0x05, 0xe4, 0xff]);
    info!("Our address = {:?}", address);

    let mut resources: HostResources<
        DefaultPacketPool,
        CONNECTIONS_MAX,
        L2CAP_CHANNELS_MAX,
        L2CAP_MTU,
    > = HostResources::new();
    let stack = trouble_host::new(controller, &mut resources)
        .set_random_address(address)
        .set_random_generator_seed(rng);
    let Host {
        mut peripheral,
        runner,
        ..
    } = stack.build();

    info!("Starting advertising and GATT service");
    let server = Server::new_with_config(GapConfig::Peripheral(
        config.peripheral_config.unwrap_or(PeripheralConfig {
            name: config.advertise_name,
            appearance: &appearance::human_interface_device::KEYBOARD,
        }),
    ));
    match server {
        Ok(server) => {
            let _ = join(ble_task(runner), async {
                loop {
                    match advertise(config.advertise_name, &mut peripheral).await {
                        Ok(conn) => {
                            let gatt_conn = conn.with_attribute_server(&server).unwrap();
                            select(
                                gatt_events_task(&gatt_conn),
                                hid_task(&server, &gatt_conn, &stack, &output_rx),
                            )
                            .await;
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

async fn ble_task<C: Controller, P: PacketPool>(mut runner: Runner<'_, C, P>) {
    loop {
        if let Err(e) = runner.run().await {
            #[cfg(feature = "defmt")]
            let e = defmt::Debug2Format(&e);
            rktk_log::error!("{:?}", e);
            return;
        }
    }
}

async fn gatt_events_task<P: PacketPool>(conn: &GattConnection<'_, '_, P>) -> Result<(), Error> {
    loop {
        match conn.next().await {
            GattConnectionEvent::Disconnected { reason } => {
                info!("[gatt] disconnected: {:?}", reason);
                break;
            }
            GattConnectionEvent::Gatt { event } => {
                let result = match &event {
                    GattEvent::Read(_event) => {
                        if conn.raw().security_level().map(|l| l.encrypted()) == Ok(true) {
                            None
                        } else {
                            Some(AttErrorCode::INSUFFICIENT_ENCRYPTION)
                        }
                    }
                    GattEvent::Write(_event) => {
                        if conn.raw().security_level().map(|l| l.encrypted()) == Ok(true) {
                            None
                        } else {
                            Some(AttErrorCode::INSUFFICIENT_ENCRYPTION)
                        }
                    }
                    _ => None,
                };

                let result = if let Some(code) = result {
                    event.reject(code)
                } else {
                    event.accept()
                };
                match result {
                    Ok(reply) => {
                        reply.send().await;
                    }
                    Err(e) => {
                        warn!("[gatt] error sending response: {:?}", e);
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
async fn advertise<'a, C: Controller, P: PacketPool>(
    name: &'a str,
    peripheral: &mut Peripheral<'a, C, P>,
) -> Result<Connection<'a, P>, BleHostError<C::Error>> {
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

async fn hid_task<C: Controller, P: PacketPool>(
    server: &Server<'_>,
    conn: &GattConnection<'_, '_, P>,
    _stack: &Stack<'_, C, P>,
    output_rx: &Receiver<'static, Report, 4>,
) {
    loop {
        let report = output_rx.receive().await;

        match report {
            Report::Keyboard(keyboard_report) => {
                let mut buf = [0u8; 8];
                if let Err(_e) = serialize(&mut buf, &keyboard_report) {
                    rktk_log::error!("failed to serialize keyboard report");
                    continue;
                }
                if let Err(e) = server.hid_service.input_keyboard.notify(conn, &buf).await {
                    rktk_log::error!("failed to send keyboard report: {:?}", e);
                    continue;
                }
            }
            Report::MediaKeyboard(media_keyboard_report) => {
                let mut buf = [0u8; 8];
                if let Err(_e) = serialize(&mut buf, &media_keyboard_report) {
                    rktk_log::error!("failed to serialize media keyboard report");
                    continue;
                }
                let usage_id: u16 = media_keyboard_report.usage_id;
                if let Err(e) = server
                    .hid_service
                    .input_media_keyboard
                    .notify(conn, &usage_id)
                    .await
                {
                    rktk_log::error!("failed to send media keyboard report: {:?}", e);
                    continue;
                }
            }
            Report::Mouse(mouse_report) => {
                let mut buf = [0u8; 5];
                if let Err(_e) = serialize(&mut buf, &mouse_report) {
                    rktk_log::error!("failed to serialize keyboard report");
                    continue;
                }
                if let Err(e) = server.hid_service.input_mouse.notify(conn, &buf).await {
                    rktk_log::error!("failed to send keyboard report: {:?}", e);
                    continue;
                }
            }
        }

        rktk_log::debug!("Successfully sent report");
    }
}
