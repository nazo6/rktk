use std::pin::pin;

use async_lock::Mutex;
use futures::future::select;
use futures::StreamExt as _;
use gloo_timers::future::TimeoutFuture;
use rktk_rrp::endpoint_client;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::HidDevice;
use web_sys::HidInputReportEvent;

pub struct HidTransportClient {
    device: HidDevice,
    input_report_receiver: Mutex<futures::channel::mpsc::UnboundedReceiver<[u8; 32]>>,
    _cb: Closure<dyn FnMut(HidInputReportEvent)>,
}

impl Drop for HidTransportClient {
    fn drop(&mut self) {
        self.device.set_oninputreport(None);
    }
}

impl HidTransportClient {
    pub fn new(device: HidDevice) -> Self {
        let (input_report_sender, input_report_receiver) = futures::channel::mpsc::unbounded();

        let cb = Closure::wrap(Box::new(move |e: HidInputReportEvent| {
            let data = e.data();
            let mut buf = [0u8; 32];
            for i in 0..32 {
                buf[i] = data.get_uint8(i);
            }

            log::info!("Report: {:X?}", buf);

            input_report_sender.unbounded_send(buf).unwrap();
        }) as Box<dyn FnMut(_)>);

        device.set_oninputreport(Some(cb.as_ref().unchecked_ref()));

        Self {
            device,
            input_report_receiver: Mutex::new(input_report_receiver),
            _cb: cb,
        }
    }

    async fn send_all(&self, buf: &[u8]) -> Result<(), anyhow::Error> {
        for chunk in buf.chunks(31) {
            let mut output_data = vec![chunk.len() as u8];
            output_data.extend_from_slice(chunk);
            output_data.resize(32, 0);

            JsFuture::from(
                self.device
                    .send_report_with_u8_slice(0, &mut output_data)
                    .map_err(|e| anyhow::anyhow!("Failed to send report: {:?}", e))?,
            )
            .await
            .map_err(|e| anyhow::anyhow!("Failed to send report: {:?}", e))?;
        }
        Ok(())
    }

    async fn read_until_zero(&self, buf: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        let mut input_report_receiver = self.input_report_receiver.lock().await;
        loop {
            if let Some(report) = input_report_receiver.next().await {
                let len = report[0] as usize;
                let data = &report[1..=len];

                buf.extend_from_slice(data);
                if data.last() == Some(&0) {
                    log::info!("Ended with: {:X?}", buf);
                    break;
                }
            }
        }

        Ok(())
    }

    endpoint_client!(
        get_keyboard_info normal normal
        get_keymaps normal stream
        get_layout_json normal stream
        set_keymaps stream normal
        get_keymap_config normal normal
        set_keymap_config normal normal
        get_log normal stream
        get_now normal normal
    );
}
