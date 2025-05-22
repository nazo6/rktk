use embassy_futures::select::{Either, select};
use postcard::{from_bytes_cobs, to_slice_cobs};
use rktk_log::{MaybeFormat, debug, helper::Debug2Format, warn};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    config::constant::CONST_CONFIG,
    drivers::interface::split::SplitDriver,
    utils::{Receiver, Sender},
};

pub const MAX_DATA_SIZE: usize = 16;

pub struct CobsReceiver {
    remained: heapless::Deque<u8, 64>,
}
impl CobsReceiver {
    fn new() -> Self {
        Self {
            remained: heapless::Deque::new(),
        }
    }

    #[inline(always)]
    async fn recv_until_zero<D: SplitDriver>(
        &mut self,
        driver: &mut D,
        buf: &mut [u8],
        is_master: bool,
    ) -> Result<usize, &'static str> {
        let mut idx = 0;

        while let Some(b) = self.remained.pop_front() {
            buf[idx] = b;
            idx += 1;
            if b == 0 {
                return Ok(idx);
            }
        }

        loop {
            let mut tmp_buf = [0u8; 64];
            let size = driver
                .recv(&mut tmp_buf, is_master)
                .await
                .map_err(|_e| "recv error")?;
            let mut end = false;
            for b in tmp_buf[..size].iter() {
                if end {
                    let _ = self.remained.push_back(*b);
                } else {
                    if let Some(buf_slot) = buf.get_mut(idx) {
                        *buf_slot = *b;
                        idx += 1;
                    } else {
                        return Err("Buffer overflow");
                    }

                    if *b == 0 {
                        end = true;
                    }
                }
            }

            if end {
                break;
            }
        }

        Ok(idx)
    }
}

/// Starts background task for master side that
/// - send data from slave to m2s channel.
/// - receive data from s2m channel and send it to slave.
pub async fn start<
    'a,
    SP: SplitDriver,
    R: DeserializeOwned + MaybeFormat,
    S: Serialize + MaybeFormat,
>(
    mut split: SP,
    received_sender: Sender<'a, R, { CONST_CONFIG.buffer.split_channel }>,
    to_send_receiver: Receiver<'a, S, { CONST_CONFIG.buffer.split_channel }>,
    is_master: bool,
) {
    debug!("split handler start");

    let mut send_id: usize = 0;
    let mut recv_id: usize = 0;
    let mut recv_err: usize = 0;

    let mut recv = CobsReceiver::new();

    loop {
        let mut recv_buf = [0u8; MAX_DATA_SIZE];

        match select(
            recv.recv_until_zero(&mut split, &mut recv_buf, is_master),
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
                            if received_sender.try_send(data).is_err() {
                                warn!("split recv chan full");
                            }
                            if id - recv_id > 1 {
                                recv_err += 1;
                                warn!(
                                    "Split communication loss detected: id:{}, err count:{}",
                                    id, recv_err
                                );
                            }
                            recv_id = id;
                        }
                        Err(e) => {
                            warn!("Split data decode failed: {:?}", Debug2Format(&e));
                        }
                    },
                    Err(e) => {
                        warn!("Failed to receive split data: {:?}", Debug2Format(&e));
                    }
                }
            }
            Either::Second(send_data) => {
                debug!("Split data send: {:?}", send_data);
                let mut send_buf = [0u8; MAX_DATA_SIZE];
                if let Ok(bytes) = to_slice_cobs(&(send_id, send_data), &mut send_buf) {
                    if let Err(e) = split.send_all(bytes, is_master).await {
                        rktk_log::error!("Split send error: {:?}", Debug2Format(&e));
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
