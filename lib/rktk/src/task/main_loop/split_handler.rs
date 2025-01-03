use embassy_futures::select::{select, Either};
use postcard::{from_bytes_cobs, to_slice_cobs};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::CONST_CONFIG,
    drivers::interface::split::SplitDriver,
    utils::{Receiver, Sender},
};

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
    received_sender: Sender<'a, R, { CONST_CONFIG.split_channel_size }>,
    to_send_receiver: Receiver<'a, S, { CONST_CONFIG.split_channel_size }>,
    is_master: bool,
) {
    let mut send_id: usize = 0;
    let mut recv_id: usize = 0;
    let mut recv_err: usize = 0;

    loop {
        let mut recv_buf = [0u8; MAX_DATA_SIZE];

        match select(
            split.wait_recv(&mut recv_buf, is_master),
            to_send_receiver.receive(),
        )
        .await
        {
            Either::First(res) => {
                // Reducing the processing time here is very important for drivers for half-duplex communication.
                // Since half-duplex communication shares one pin between Rx and Tx, this means that in most cases the communication peripheral must be initialized for each receive or transmit.
                // This process takes some time, and the longer this block takes, the higher the error rate will be at higher communication speeds.
                //
                // I measured the time in this block using embassy-time, and while it was on the order of microseconds, I do not believe this is an accurate figure.
                // In fact, removing rktk::print from this block improved the error rate.
                match res {
                    Ok(_) => match from_bytes_cobs::<(usize, R)>(&mut recv_buf.clone()) {
                        Ok((id, data)) => {
                            let _ = received_sender.try_send(data);
                            if id - recv_id > 1 {
                                recv_err += 1;
                                log::warn!(
                                    "Split communication loss detected: id:{}, err count:{}",
                                    id,
                                    recv_err
                                );
                            }
                            recv_id = id;
                        }
                        Err(e) => {
                            log::warn!("Split data decode failed: {:?}", e);
                        }
                    },
                    Err(e) => log::warn!("Failed to receive split data: {:?}", e),
                }
            }
            Either::Second(send_data) => {
                let mut send_buf = [0u8; MAX_DATA_SIZE];
                if let Ok(bytes) = to_slice_cobs(&(send_id, send_data), &mut send_buf) {
                    if let Err(e) = split.send(bytes, is_master).await {
                        log::error!("Split send error: {:?}", e)
                    }
                    send_id += 1;
                }
            }
        }
    }
}

// fn fmt_array(arr: &[u8]) -> heapless::String<64> {
//     use core::fmt::Write as _;
//
//     let mut str = heapless::String::<64>::new();
//
//     for b in arr {
//         let _ = write!(str, "{:02X}", b);
//     }
//     str
// }
