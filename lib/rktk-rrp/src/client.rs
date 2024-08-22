#[macro_export]
macro_rules! endpoint_client {
    ($($ep:ident $req_type:tt $res_type:tt)*) => {
        $(
            endpoint_client!(@def $ep $req_type $res_type);
        )*
    };

    (@def $ep:ident normal normal) => {
        pub async fn $ep(&mut self, req: $crate::endpoints::$ep::Request) -> Result<$crate::endpoints::$ep::Response, anyhow::Error> {
            use $crate::endpoints::$ep::{Request, Response};
            endpoint_client!(@send_ep_name self, $ep);
            endpoint_client!(@send_req normal self, $ep, req);
            Ok(endpoint_client!(@get_res normal self, $ep))
        }
    };
    (@def $ep:ident stream normal) => {
        pub async fn $ep(&mut self, req: impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamRequest>) ->
            Result<$crate::endpoints::$ep::Response, anyhow::Error>
        {
            use $crate::endpoints::$ep::{StreamRequest, Response};
            endpoint_client!(@send_ep_name self, $ep);
            endpoint_client!(@send_req stream self, $ep, req);
            Ok(endpoint_client!(@get_res normal self, $ep))
        }
    };
    (@def $ep:ident normal stream) => {
        pub async fn $ep(&mut self, req: $crate::endpoints::$ep::Request)
            -> Result<impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamResponse> + '_, anyhow::Error>
        {
            endpoint_client!(@send_ep_name self, $ep);
            use $crate::endpoints::$ep::{Request, StreamResponse};
            endpoint_client!(@send_req normal self, $ep, req);
            let res = endpoint_client!(@get_res stream self, $ep);
            Ok(res)
        }
    };
    (@def $ep:ident stream stream) => {
        pub async fn $ep(&mut self, req: impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamRequest>)
            -> Result<impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamResponse>, anyhow::Error>
        {
            use $crate::endpoints::$ep::{StreamRequest, StreamResponse};
            endpoint_client!(@send_ep_name self, $ep);
            endpoint_client!(@send_req stream self, $ep, req);
            Ok(endpoint_client!(@get_res stream self, $ep))
        }
    };

    (@send_ep_name $self:expr, $ep:ident) => {
        use $crate::__reexports::postcard::experimental::max_size::MaxSize;
        use $crate::__reexports::postcard::from_bytes_cobs;
        use $crate::__reexports::postcard::to_stdvec_cobs;
        use $crate::__reexports::heapless::String;
        use anyhow::Context as _;

        let name = stringify!($ep);
        let name_buf = to_stdvec_cobs(&name).with_context(|| format!("Failed to serialize ep name: {}", name))?;
        $self.send_all(&name_buf).await.with_context(|| format!("Failed to send ep name: {}", name))?;
    };

    (@get_res normal $self:expr, $ep:ident) => {{
        let mut buf = vec![];
        let _ = $self.read_until_zero(&mut buf).await.with_context(|| format!("Failed to receive response of ep {}", stringify!($ep)))?;
        let req = from_bytes_cobs::<Response>(&mut buf).with_context(|| format!("Failed to deserialize response of ep {}", stringify!($ep)))?;
        req
    }};
    (@get_res stream $et:expr, $ep:ident) => {{
        use $crate::__reexports::futures::stream;

        stream::unfold((), |state| async {
            let mut buf = vec![];
            loop {
                let Ok(size) = $et.read_until_zero(&mut buf).await else {
                    continue;
                };
                if buf.len() == 1 {
                    return None;
                }
                let Ok(req) = from_bytes_cobs::<StreamResponse>(&mut buf) else {
                    continue;
                };
                return Some((req, ()));
            }
        })
    }};

    (@send_req normal $et:expr, $ep:ident, $data:expr) => {{
        let res: Request = $data;
        let res = to_stdvec_cobs(&res).with_context(|| format!("Failed to serialize request ({})", stringify!($ep)))?;
        $et.send_all(&res).await.context("Failed to send request")?;
    }};

    (@send_req stream $et:expr, $ep:ident, $data:expr) => {{
        use $crate::__reexports::futures::stream::StreamExt;
        let mut data = core::pin::pin!($data);
        while let Some(res) = data.next().await {
            let res: StreamRequest = res;
            let Ok(res) = to_stdvec_cobs(&res) else {
                continue;
            };
            let _ = $et.send_all(&res).await;
        }
        $et.send_all(&[0x00]).await.context("Failed to send request")?;
    }};
}
