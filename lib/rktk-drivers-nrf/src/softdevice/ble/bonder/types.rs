use nrf_softdevice::ble::{Address, EncryptionInfo, MasterId};
use serde::{Deserialize, Serialize};

use super::MAX_PEER_COUNT;

#[derive(Serialize, Deserialize)]
#[serde(remote = "MasterId")]
struct MasterIdDef {
    pub ediv: u16,
    pub rand: [u8; 8],
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "EncryptionInfo")]
pub struct EncryptionInfoDef {
    pub ltk: [u8; 16],
    pub flags: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceData {
    /// peer_addr is not needed to store because it is random and changes every time.
    /// Instead, in get_key, we can use peer_address to get the peer data.
    #[serde(skip)]
    pub peer_addr: Option<Address>,
    #[serde(with = "MasterIdDef")]
    pub master_id: MasterId,
    #[serde(with = "EncryptionInfoDef")]
    pub encryption_info: EncryptionInfo,
    pub sys_attrs: Option<heapless::Vec<u8, 62>>,
}

pub type Devices = heapless::Vec<DeviceData, MAX_PEER_COUNT>;
