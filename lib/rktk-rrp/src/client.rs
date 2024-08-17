#[macro_export]
macro_rules! endpoint_client {
    ($($ep:ident $req_type:tt $res_type:tt)*) => {
        $(
            endpoint_client!(@def $ep $req_type $res_type);
        )*
    };

    (@def $ep:ident normal normal) => {
        pub async fn $ep(&self, req: $crate::endpoints::$ep::Request) -> Result<$crate::endpoints::$ep::Response, anyhow::Error> {
            use $crate::endpoints::$ep::{Request, Response};
            endpoint_client!(@send_ep_name self, $ep);
            endpoint_client!(@send_req normal self, $ep, req);
            Ok(endpoint_client!(@get_res normal self, $ep))
        }
    };
    (@def $ep:ident stream normal) => {
        pub async fn $ep(&self, req: impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamRequest>) ->
            Result<$crate::endpoints::$ep::Response, anyhow::Error>
        {
            use $crate::endpoints::$ep::{StreamRequest, Response};
            endpoint_client!(@send_ep_name self, $ep);
            endpoint_client!(@send_req stream self, $ep, req);
            Ok(endpoint_client!(@get_res normal self, $ep))
        }
    };
    (@def $ep:ident normal stream) => {
        pub async fn $ep(&self, req: $crate::endpoints::$ep::Request)
            -> Result<impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamResponse> + '_, anyhow::Error>
        {
            endpoint_client!(@send_ep_name self, $ep);
            use $crate::endpoints::$ep::{Request, StreamResponse};
            endpoint_client!(@send_req normal self, $ep, req);
            Ok(endpoint_client!(@get_res stream self, $ep))
        }
    };
    (@def $ep:ident stream stream) => {
        pub async fn $ep(&self, req: impl $crate::__reexports::futures::stream::Stream<Item = $crate::endpoints::$ep::StreamRequest>)
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
        use $crate::__reexports::postcard::to_slice_cobs;
        use $crate::__reexports::heapless::String;

        let mut name_buf = [0u8; 64];
        to_slice_cobs(&stringify!($ep), &mut name_buf)?;
        $self.send_all(&mut name_buf).await?;
    };

    (@get_res normal $self:expr, $ep:ident) => {{
        let mut buf = [0u8; Response::POSTCARD_MAX_SIZE + Response::POSTCARD_MAX_SIZE / 254 + 2];
        let _ = $self.read_until_zero(&mut buf).await;
        let req = from_bytes_cobs::<Response>(&mut buf)?;
        req
    }};
    (@get_res stream $et:expr, $ep:ident) => {{
        use $crate::__reexports::futures::stream;

        stream::unfold((), |state| async {
            let mut buf = [0u8; StreamResponse::POSTCARD_MAX_SIZE + StreamResponse::POSTCARD_MAX_SIZE / 254 + 2];
            loop {
                let Ok(size) = $et.read_until_zero(&mut buf).await else {
                    continue;
                };
                if size == 1 {
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
        let mut buf = [0u8; Request::POSTCARD_MAX_SIZE + Request::POSTCARD_MAX_SIZE / 254 + 2];
        let res = to_slice_cobs(&res, &mut buf)?;
        let _ = $et.send_all(res).await;
    }};

    (@send_req stream $et:expr, $ep:ident, $data:expr) => {{
        use $crate::futures::stream::StreamExt;
        while let Some(res) = $data.next().await {
            let res: StreamRequest = res;
            let mut buf = [0u8; StreamRequest::POSTCARD_MAX_SIZE + StreamRequest::POSTCARD_MAX_SIZE / 254 + 2];
            let Ok(res) = to_slice_cobs(&res, &mut buf) else {
                continue;
            };
            let _ = $et.send_all(res).await;
        }
        let _ = $et.send_all(&[0x00]).await;
    }};
}
