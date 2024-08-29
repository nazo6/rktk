use futures::future::select;
use gloo_timers::future::TimeoutFuture;
use rktk_rrp::endpoint_client;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{js_sys, SerialPort};

pub struct SerialClient {
    pub(crate) stream: SerialPort,
}

impl SerialClient {
    async fn send_all(&self, buf: &[u8]) -> Result<(), anyhow::Error> {
        let writer = self
            .stream
            .writable()
            .get_writer()
            .map_err(|e| anyhow::anyhow!("Failed to get writer: {:?}", e))?;
        let r: Result<(), anyhow::Error> = async {
            JsFuture::from(writer.ready())
                .await
                .map_err(|e| anyhow::anyhow!("Failed to wait writer for ready: {:?}", e))?;
            JsFuture::from(writer.write_with_chunk(&buf.to_owned().into()))
                .await
                .map_err(|e| anyhow::anyhow!("Failed to write: {:?}", e))?;
            JsFuture::from(writer.ready())
                .await
                .map_err(|e| anyhow::anyhow!("Failed to wait writer for ready: {:?}", e))?;
            Ok(())
        }
        .await;

        writer.release_lock();

        r?;

        Ok(())
    }
    async fn read_until_zero(&self, buf: &mut Vec<u8>) -> Result<(), anyhow::Error> {
        let reader_options = web_sys::ReadableStreamGetReaderOptions::new();
        reader_options.set_mode(web_sys::ReadableStreamReaderMode::Byob);
        let reader = self
            .stream
            .readable()
            .get_reader_with_options(&reader_options)
            .dyn_into::<web_sys::ReadableStreamByobReader>()
            .expect("Invalid readable stream");

        let res = match select(
            std::pin::pin!(async {
                loop {
                    let typed_array = web_sys::js_sys::Uint8Array::new(&JsValue::from(1));
                    let promise = reader.read_with_array_buffer_view(&typed_array);
                    let obj = JsFuture::from(promise)
                        .await
                        .map_err(|e| anyhow::anyhow!("Failed to read data: {:?}", e))?;

                    let done = js_sys::Reflect::get(&obj, &JsValue::from("done"))
                        .expect("Failed to get done property of data")
                        .as_bool()
                        .unwrap_or(false);
                    if done {
                        Err(anyhow::anyhow!("EOF"))?;
                    } else {
                        let array = js_sys::Reflect::get(&obj, &JsValue::from("value"))
                            .expect("Failed to get value property of data")
                            .dyn_into::<js_sys::Uint8Array>()
                            .expect("Expected Uint8Array");

                        let val = array.get_index(0);
                        buf.push(val);
                        if val == 0 {
                            break;
                        }
                    }
                }
                Result::<(), anyhow::Error>::Ok(())
            }),
            TimeoutFuture::new(500),
        )
        .await
        {
            futures::future::Either::Left((res, _)) => res,
            futures::future::Either::Right(_) => Err(anyhow::anyhow!("Timeout")),
        };

        reader.release_lock();

        res?;

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
