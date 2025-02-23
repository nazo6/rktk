use super::RrpHidBackend;
use super::RrpHidDevice;
use anyhow::Context as _;
use futures::StreamExt as _;
use rktk_rrp::client::Client;
use rktk_rrp::transport::ReadTransport;
use rktk_rrp::transport::WriteTransport;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::HidDevice;
use web_sys::HidInputReportEvent;

pub struct WebHidBackend {}

impl RrpHidBackend for WebHidBackend {
    type Error = anyhow::Error;

    type HidDevice = WebHidDevice;

    fn new() -> Self {
        todo!()
    }

    async fn open_device(
        &mut self,
        usage_page: u16,
        usage: u16,
    ) -> Result<Self::HidDevice, Self::Error> {
        let window = web_sys::window().context("Missing Window")?;
        let hid = window.navigator().hid();

        let devices_promise = hid.request_device(&web_sys::HidDeviceRequestOptions::new(
            &serde_wasm_bindgen::to_value(&[Filter {
                usage_page: Some(usage_page),
                usage: Some(usage),
            }])
            .unwrap(),
        ));
        let devices = wasm_bindgen_futures::JsFuture::from(devices_promise)
            .await
            .map_err(|e| anyhow::anyhow!("Cannot get devices: {:?}", e))?;
        let devs_array = devices
            .dyn_ref::<js_sys::Array>()
            .context("Cannot get devices")?;
        let device: JsValue = devs_array.at(0);
        let device: HidDevice = device
            .dyn_into()
            .map_err(|_| anyhow::anyhow!("No device selected"))?;
        wasm_bindgen_futures::JsFuture::from(device.open())
            .await
            .map_err(|e| anyhow::anyhow!("Cannot open device: {:?}", e))?;

        let client = Client::new(
            HidReader::new(device.clone()),
            HidWriter::new(device.clone()),
        );

        Ok(Self::HidDevice { client, device })
    }

    fn set_ondisconnect(&mut self, fun: Option<impl FnMut() + 'static>) {
        let window = web_sys::window().expect("Missing Window");
        let hid = window.navigator().hid();

        if let Some(fun) = fun {
            let cb = Closure::wrap(Box::new(fun) as Box<dyn FnMut()>);
            hid.set_ondisconnect(Some(cb.as_ref().unchecked_ref()));
        } else {
            hid.set_ondisconnect(None);
        };
    }
}

#[allow(non_snake_case)]
#[derive(serde::Serialize)]
struct Filter {
    #[serde(rename = "usagePage")]
    pub usage_page: Option<u16>,
    pub usage: Option<u16>,
}

pub struct WebHidDevice {
    client: Client<HidReader, HidWriter>,
    device: HidDevice,
}

impl RrpHidDevice for WebHidDevice {
    type Error = anyhow::Error;

    type ReadTransport = HidReader;

    type WriteTransport = HidWriter;

    fn get_client(&mut self) -> &mut Client<Self::ReadTransport, Self::WriteTransport> {
        &mut self.client
    }

    async fn close(&mut self) -> Result<(), Self::Error> {
        JsFuture::from(self.device.close())
            .await
            .map_err(|e| anyhow::anyhow!("Failed to close device: {:?}", e))?;
        Ok(())
    }
}

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

impl ReadTransport for HidReader {
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

impl WriteTransport for HidWriter {
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
