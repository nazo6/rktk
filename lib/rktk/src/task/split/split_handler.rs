use embassy_futures::select::{select, Either};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Receiver, Sender},
};
use postcard::{from_bytes, to_slice};
use serde::{de::DeserializeOwned, Serialize};

use crate::config::SPLIT_CHANNEL_SIZE;

use super::super::split::*;

pub const MAX_DATA_SIZE: usize = 8;

/// Starts background task for master side that
/// - send data from slave to m2s channel.
/// - receive data from s2m channel and send it to slave.
pub async fn start<'a, SP: SplitDriver, R: DeserializeOwned, S: Serialize>(
    mut split: SP,
    received_sender: Sender<'a, CriticalSectionRawMutex, R, SPLIT_CHANNEL_SIZE>,
    to_send_receiver: Receiver<'a, CriticalSectionRawMutex, S, SPLIT_CHANNEL_SIZE>,
) {
    let mut recv_buf = [0u8; MAX_DATA_SIZE];
    let mut send_buf = [0u8; MAX_DATA_SIZE];

    loop {
        match select(split.wait_recv(&mut recv_buf), to_send_receiver.receive()).await {
            Either::First(_) => {
                if let Ok(data) = from_bytes(&recv_buf) {
                    crate::print!("RecvOk: {}", embassy_time::Instant::now());
                    let _ = received_sender.send(data).await;
                } else {
                    crate::print!("RecvErr: {}", embassy_time::Instant::now());
                }
            }
            Either::Second(send_data) => {
                if let Ok(bytes) = to_slice(&send_data, &mut send_buf) {
                    let _ = split.send(bytes).await;
                }
            }
        }
    }
}
