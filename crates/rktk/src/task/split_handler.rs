use embassy_futures::select::{Either, select};
use postcard::{experimental::max_size::MaxSize, from_bytes_cobs, to_slice_cobs};
use rktk_log::{MaybeFormat, debug, helper::Debug2Format, warn};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    config::CONST_CONFIG,
    drivers::interface::split::{MasterToSlave, SlaveToMaster, SplitDriver},
    utils::{Receiver, Sender},
};

pub const MAX_DATA_SIZE: usize = const {
    if MasterToSlave::POSTCARD_MAX_SIZE > SlaveToMaster::POSTCARD_MAX_SIZE {
        MasterToSlave::POSTCARD_MAX_SIZE
    } else {
        SlaveToMaster::POSTCARD_MAX_SIZE
    }
};

pub struct CobsReceiver {
    buf: heapless::Vec<u8, 128>,
}
impl CobsReceiver {
    fn new() -> Self {
        Self { buf: heapless::Vec::new() }
    }

    #[inline(always)]
    async fn recv_until_zero<D: SplitDriver>(
        &mut self,
        driver: &mut D,
        out_buf: &mut [u8],
        is_master: bool,
    ) -> Result<usize, &'static str> {
        if let Some(zero_pos) = self.buf.iter().position(|&b| b == 0) {
            let len = zero_pos + 1;
            if len > out_buf.len() {
                return Err("Output buffer too small");
            }
            out_buf[..len].copy_from_slice(&self.buf[..len]);

            // Shift remaining bytes
            self.buf.copy_within(len.., 0);
            self.buf.truncate(self.buf.len() - len);
            return Ok(len);
        }

        loop {
            let mut tmp_buf = [0u8; 64];
            let size = driver.recv(&mut tmp_buf, is_master).await.map_err(|_e| "recv error")?;

            if size == 0 {
                continue;
            }

            for &b in &tmp_buf[..size] {
                if self.buf.push(b).is_err() {
                    return Err("CobsReceiver buffer overflow");
                }
            }

            if let Some(zero_pos) = self.buf.iter().position(|&b| b == 0) {
                let len = zero_pos + 1;
                if len > out_buf.len() {
                    return Err("Output buffer too small");
                }
                out_buf[..len].copy_from_slice(&self.buf[..len]);

                // Shift remaining bytes
                self.buf.copy_within(len.., 0);
                self.buf.truncate(self.buf.len() - len);
                return Ok(len);
            }
        }
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
            Either::First(res) => match res {
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
            },
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
