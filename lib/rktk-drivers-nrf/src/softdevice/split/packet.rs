#![allow(clippy::manual_div_ceil)]

use core::ptr::NonNull;

use atomic_pool::{pool, Box};
use nrf_softdevice::ble::l2cap;

pool!(pub PacketPool: [[u8; 64]; 10]);

#[derive(Debug)]
pub struct Packet {
    len: u8,
    buf: Box<PacketPool>,
}

impl Packet {
    pub fn new(data: &[u8]) -> Self {
        let Some(mut buf) = Box::<PacketPool>::new([0; 64]) else {
            panic!("PacketPool allocation failed");
        };
        buf[..data.len()].copy_from_slice(data);
        Packet {
            len: data.len() as u8,
            buf,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len as usize]
    }
}

impl l2cap::Packet for Packet {
    const MTU: usize = 64;

    fn allocate() -> Option<NonNull<u8>> {
        if let Some(buf) = Box::<PacketPool>::new([0; 64]) {
            let ptr = Box::into_raw(buf).cast::<u8>();
            // info!("allocate {}", ptr.as_ptr() as u32);
            Some(ptr)
        } else {
            None
        }
    }

    fn into_raw_parts(self) -> (NonNull<u8>, usize) {
        let ptr = Box::into_raw(self.buf).cast::<u8>();
        let len = self.len;
        // info!("into_raw_parts {}", ptr.as_ptr() as u32);
        (ptr, len as usize)
    }

    unsafe fn from_raw_parts(ptr: NonNull<u8>, len: usize) -> Self {
        // info!("from_raw_parts {}", ptr.as_ptr() as u32);
        Self {
            len: len as u8,
            buf: Box::from_raw(ptr.cast::<[u8; 64]>()),
        }
    }
}
