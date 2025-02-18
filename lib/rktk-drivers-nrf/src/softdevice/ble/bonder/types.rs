use nrf_softdevice::{
    ble::{Address, EncryptionInfo, IdentityKey, IdentityResolutionKey, MasterId},
    raw::ble_gap_irk_t,
};
use rktk_log::derive_format_and_debug;
use serde::{Deserialize, Serialize};

use super::MAX_PEER_COUNT;

#[derive_format_and_debug]
#[derive(Serialize, Deserialize)]
#[serde(remote = "MasterId")]
pub struct MasterIdDef {
    pub ediv: u16,
    pub rand: [u8; 8],
}

#[derive_format_and_debug]
#[derive(Serialize, Deserialize)]
#[serde(remote = "EncryptionInfo")]
pub struct EncryptionInfoDef {
    pub ltk: [u8; 16],
    pub flags: u8,
}

#[derive_format_and_debug]
#[derive(Serialize, Deserialize)]
#[serde(remote = "Address")]
pub struct AddressDef {
    pub flags: u8,
    pub bytes: [u8; 6],
}

#[derive_format_and_debug]
#[derive(Serialize, Deserialize)]
#[serde(remote = "IdentityResolutionKey")]
pub struct IdentityResolutionKeyDef {
    #[serde(getter = "get_irk")]
    pub irk: [u8; 16],
}

fn get_irk(irk: &IdentityResolutionKey) -> [u8; 16] {
    irk.as_raw().irk
}

impl From<IdentityResolutionKeyDef> for IdentityResolutionKey {
    fn from(irk: IdentityResolutionKeyDef) -> Self {
        IdentityResolutionKey::from_raw(ble_gap_irk_t { irk: irk.irk })
    }
}

#[derive_format_and_debug]
#[derive(Serialize, Deserialize)]
#[serde(remote = "IdentityKey")]
pub struct IdentityKeyDef {
    #[serde(with = "IdentityResolutionKeyDef")]
    pub irk: IdentityResolutionKey,
    #[serde(with = "AddressDef")]
    pub addr: Address,
}

#[derive_format_and_debug]
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DeviceData {
    #[serde(with = "IdentityKeyDef")]
    pub peer_id: IdentityKey,
    #[serde(with = "MasterIdDef")]
    pub master_id: MasterId,
    #[serde(with = "EncryptionInfoDef")]
    pub encryption_info: EncryptionInfo,
    pub sys_attrs: Option<heapless::Vec<u8, 62>>,
}

pub type Devices = heapless::Vec<DeviceData, MAX_PEER_COUNT>;
