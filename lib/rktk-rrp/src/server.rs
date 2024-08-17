#[allow(async_fn_in_trait)]
pub trait EndpointTransport {
    type Error;
    /// Wait and read serial data until a zero byte is received
    /// Implementations should not consume bytes after the zero byte
    async fn read_until_zero(&self, buf: &mut [u8]) -> Result<usize, Self::Error>;
    /// Send all bytes in the buffer.
    async fn send_all(&self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! endpoint_server {
    ($($ep:ident $req_type:tt $res_type:tt => $handler:ident)*) => {
        pub async fn handle<ET: EndpointTransport>(&mut self, et: &ET) {
            use postcard::from_bytes_cobs;
            use postcard::experimental::max_size::MaxSize;

            loop {
                let mut name_buf = [0u8; 64];
                let _ = et.read_until_zero(&mut name_buf).await;
                let Ok(ep_name) = from_bytes_cobs::<heapless::String<64>>(&mut name_buf) else {
                    continue;
                };

                match ep_name.as_str() {
                    $(
                        stringify!($ep) => {
                            use rktk_rrp::endpoints::$ep::*;
                            let req = endpoint_server!(@get $req_type et, $ep);
                            let mut res = self.$handler(req).await;
                            endpoint_server!(@send $res_type et, $ep, res);
                        }
                    )*
                    _ => {}
                }
            }
        }
    };
    (@get normal $et:expr, $ep:ident) => {{
        let mut buf = [0u8; Request::POSTCARD_MAX_SIZE + Request::POSTCARD_MAX_SIZE / 254 + 2];
        let _ = $et.read_until_zero(&mut buf).await;
        let Ok(req) = postcard::from_bytes_cobs::<Request>(&mut buf) else {
            continue;
        };
        req
    }};
    (@get stream $et:expr, $ep:ident) => {{
        use $crate::__reexports::futures::stream;
        let mut buf = [0u8; StreamRequest::POSTCARD_MAX_SIZE + StreamRequest::POSTCARD_MAX_SIZE / 254 + 2];

        stream::unfold((), |state| async {
            let mut buf = [0u8; StreamRequest::POSTCARD_MAX_SIZE + StreamRequest::POSTCARD_MAX_SIZE / 254 + 2];
            loop {
                let Ok(size) = $et.read_until_zero(&mut buf).await else {
                    continue;
                };
                if size == 1 {
                    return None;
                }
                let Ok(req) = postcard::from_bytes_cobs::<StreamRequest>(&mut buf) else {
                    continue;
                };
                return Some((req, ()));
            }
        })
    }};

    (@send normal $et:expr, $ep:ident, $data:expr) => {{
        let res: Response = $data;
        let mut buf = [0u8; Response::POSTCARD_MAX_SIZE + Response::POSTCARD_MAX_SIZE / 254 + 2];
        let Ok(res) = postcard::to_slice_cobs(&res, &mut buf) else {
            continue;
        };
        let _ = $et.send_all(res).await;
    }};

    (@send stream $et:expr, $ep:ident, $data:expr) => {{
        use $crate::__reexports::futures::stream::StreamExt;
        while let Some(res) = $data.next().await {
            let res: StreamResponse = res;
            let mut buf = [0u8; StreamResponse::POSTCARD_MAX_SIZE + StreamResponse::POSTCARD_MAX_SIZE / 254 + 2];
            let Ok(res) = postcard::to_slice_cobs(&res, &mut buf) else {
                continue;
            };
            let _ = $et.send_all(res).await;
        }
        let _ = $et.send_all(&[0x00]).await;
    }};
}
