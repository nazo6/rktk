use nrf_softdevice::{
    ble::{
        gatt_server::{self, RegisterError, WriteOp},
        Connection,
    },
    Softdevice,
};

use super::services::{
    battery::BatteryService,
    device_information::{DeviceInformation, DeviceInformationService, PnPID, VidSource},
    hid::HidService,
};

pub struct Server {
    pub _dis: DeviceInformationService,
    pub bas: BatteryService,
    pub hid: HidService,
}

impl Server {
    pub fn new(
        sd: &mut Softdevice,
        device_information: DeviceInformation,
    ) -> Result<Self, RegisterError> {
        let dis = DeviceInformationService::new(
            sd,
            &PnPID {
                vid_source: VidSource::UsbIF,
                vendor_id: 0xDEAD,
                product_id: 0xBEEF,
                product_version: 0x0000,
            },
            device_information,
        )?;

        let bas = BatteryService::new(sd)?;
        let _ = bas.battery_level_set(sd, 100);

        let hid = HidService::new(sd)?;

        Ok(Self {
            _dis: dis,
            bas,
            hid,
        })
    }
}

impl gatt_server::Server for Server {
    type Event = ();

    fn on_write(
        &self,
        conn: &Connection,
        handle: u16,
        _op: WriteOp,
        _offset: usize,
        data: &[u8],
    ) -> Option<Self::Event> {
        self.hid.on_write(conn, handle, data);
        self.bas.on_write(handle, data);
        None
    }
}
