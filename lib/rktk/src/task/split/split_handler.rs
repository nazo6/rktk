use embassy_futures::select::{select, Either};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Receiver, Sender},
};
use postcard::{from_bytes_cobs, to_slice_cobs};
use serde::{de::DeserializeOwned, Serialize};

use crate::config::SPLIT_CHANNEL_SIZE;

use super::super::split::*;

pub const MAX_DATA_SIZE: usize = 16;

/// Starts background task for master side that
/// - send data from slave to m2s channel.
/// - receive data from s2m channel and send it to slave.
pub async fn start<
    'a,
    SP: SplitDriver,
    R: DeserializeOwned + core::fmt::Debug,
    S: Serialize + core::fmt::Debug,
>(
    mut split: SP,
    received_sender: Sender<'a, CriticalSectionRawMutex, R, SPLIT_CHANNEL_SIZE>,
    to_send_receiver: Receiver<'a, CriticalSectionRawMutex, S, SPLIT_CHANNEL_SIZE>,
    is_master: bool,
) {
    let mut recv_buf = [0u8; MAX_DATA_SIZE];
    let mut send_buf = [0u8; MAX_DATA_SIZE];

    loop {
        match select(
            split.wait_recv(&mut recv_buf, is_master),
            to_send_receiver.receive(),
        )
        .await
        {
            Either::First(res) => {
                if let Err(e) = res {
                    crate::print!("RER: {:?} {}", e, embassy_time::Instant::now());
                } else if let Ok(data) = from_bytes_cobs(&mut recv_buf) {
                    crate::print!("R: {:?} {}", data, embassy_time::Instant::now());
                    let _ = received_sender.send(data).await;
                } else {
                    crate::print!(
                        "REP:{} {}",
                        fmt_array(&recv_buf),
                        embassy_time::Instant::now()
                    );
                }
            }
            Either::Second(send_data) => {
                if let Ok(bytes) = to_slice_cobs(&send_data, &mut send_buf) {
                    if let Err(e) = split.send(bytes, is_master).await {
                        crate::print!("SE: {:?} {}", e, embassy_time::Instant::now())
                    }
                }
            }
        }
    }
}

fn fmt_array(arr: &[u8]) -> heapless::String<64> {
    use core::fmt::Write as _;

    let mut str = heapless::String::<64>::new();

    for b in arr {
        let _ = write!(str, "{:02X}", b);
    }
    str
}
