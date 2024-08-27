#[macro_export]
macro_rules! endpoint_client {
    ($($endpoint:ident $req_type:tt $res_type:tt)*) => {
        $(
            endpoint_client!(@def $endpoint $req_type $res_type);
        )*
    };

    (@req_type normal, $endpoint:ident) => {$crate::endpoints::$endpoint::Request};
    (@req_type stream, $endpoint:ident) => {impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$endpoint::StreamRequest>};

    (@res_type normal, $endpoint:ident) => {$crate::endpoints::$endpoint::Response};
    (@res_type stream, $endpoint:ident) => {impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$endpoint::StreamResponse> + '_};

    (@def $endpoint:ident $req_type:tt $res_type:tt) => {
        pub async fn $endpoint(&mut self, req: endpoint_client!(@req_type $req_type, $endpoint)) -> Result<endpoint_client!(@res_type $res_type, $endpoint), anyhow::Error> {
            use $crate::__reexports::postcard::experimental::max_size::MaxSize;
            use $crate::__reexports::postcard::from_bytes_cobs;
            use $crate::__reexports::postcard::to_stdvec_cobs;
            use $crate::__reexports::heapless::String;
            use anyhow::Context as _;

            endpoint_client!(@send_ep_name, self, $endpoint);
            endpoint_client!(@send_req $req_type, self, $endpoint, req);
            Ok(endpoint_client!(@get_res $res_type, self, $endpoint))
        }
    };

    (@send_ep_name, $transport:expr, $endpoint:ident) => {
        let name = stringify!($endpoint);
        let name_buf = to_stdvec_cobs(&name).with_context(|| format!("Failed to serialize ep name: {}", name))?;
        $transport.send_all(&name_buf).await.with_context(|| format!("Failed to send ep name: {}", name))?;
    };

    (@get_res normal, $transport:expr, $endpoint:ident) => {{
        let mut buf = vec![];
        let _ = $transport.read_until_zero(&mut buf).await.with_context(|| format!("Failed to receive response of ep {}", stringify!($endpoint)))?;
        let req = from_bytes_cobs::<$crate::endpoints::$endpoint::Response>(&mut buf)
            .with_context(|| format!("Failed to deserialize response of ep {}", stringify!($endpoint)))?;
        req
    }};
    (@get_res stream, $et:expr, $endpoint:ident) => {{
        use $crate::__reexports::futures::stream;

        stream::unfold((), |state| async {
            let mut buf = vec![];
            loop {
                let Ok(size) = $et.read_until_zero(&mut buf).await else {
                    break;
                };
                if buf.len() == 1 {
                    break;
                }
                let Ok(req) = from_bytes_cobs::<$crate::endpoints::$endpoint::StreamResponse>(&mut buf) else {
                    break;
                };
                return Some((req, ()));
            }
            return None;
        })
    }};

    (@send_req normal, $et:expr, $endpoint:ident, $data:expr) => {{
        let res: $crate::endpoints::$endpoint::Request = $data;
        let res = to_stdvec_cobs(&res).with_context(|| format!("Failed to serialize request ({})", stringify!($endpoint)))?;
        $et.send_all(&res).await.context("Failed to send request")?;
    }};
    (@send_req stream, $et:expr, $endpoint:ident, $data:expr) => {{
        use $crate::__reexports::futures::stream::StreamExt;
        let mut data = core::pin::pin!($data);
        while let Some(res) = data.next().await {
            let res: $crate::endpoints::$endpoint::StreamRequest = res;
            let Ok(res) = to_stdvec_cobs(&res) else {
                continue;
            };
            let _ = $et.send_all(&res).await;
        }
        $et.send_all(&[0x00]).await.context("Failed to send request")?;
    }};
}
