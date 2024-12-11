use futures::StreamExt as _;
use rktk_rrp::transport::ReadTransport;
use rktk_rrp::transport::WriteTransport;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HidDevice;
use web_sys::HidInputReportEvent;

pub struct HidReader {
    device: HidDevice,
    pipe_recv: futures::channel::mpsc::UnboundedReceiver<u8>,
    _cb: Closure<dyn FnMut(HidInputReportEvent)>,
}

impl Drop for HidReader {
    fn drop(&mut self) {
        self.device.set_oninputreport(None);
    }
}

impl HidReader {
    pub fn new(device: HidDevice) -> Self {
        let (pipe_send, pipe_recv) = futures::channel::mpsc::unbounded();

        let cb = Closure::wrap(Box::new(move |e: HidInputReportEvent| {
            let data = e.data();
            let mut buf = [0u8; 32];
            for (i, byte) in buf.iter_mut().enumerate() {
                *byte = data.get_uint8(i);
            }
            let size = buf[0] as usize;
            for i in 0..size {
                pipe_send.unbounded_send(buf[i + 1]).unwrap();
            }
        }) as Box<dyn FnMut(_)>);

        device.set_oninputreport(Some(cb.as_ref().unchecked_ref()));

        Self {
            device,
            pipe_recv,
            _cb: cb,
        }
    }
}

impl<const BUF_SIZE: usize> ReadTransport<BUF_SIZE> for HidReader {
    type Error = String;

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut i = 0;
        while i < buf.len() {
            if let Some(data) = self.pipe_recv.next().await {
                buf[i] = data;
                i += 1;
            } else {
                return Err("Read failed".to_string());
            }
        }
        Ok(i)
    }
}

pub struct HidWriter {
    device: HidDevice,
}

impl HidWriter {
    pub fn new(device: HidDevice) -> Self {
        Self { device }
    }
}

impl<const BUF_SIZE: usize> WriteTransport<BUF_SIZE> for HidWriter {
    type Error = String;

    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        for chunk in buf.chunks(31) {
            let mut data = vec![chunk.len() as u8];
            data.extend_from_slice(chunk);
            data.resize(32, 0);
            let p = self
                .device
                .send_report_with_u8_slice(0, &mut data)
                .map_err(|e| format!("{:?}", e))?;
            wasm_bindgen_futures::JsFuture::from(p)
                .await
                .map_err(|e| format!("{:?}", e))?;
        }

        Ok(buf.len())
    }
}
